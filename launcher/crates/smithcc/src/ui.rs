//! Dark suite window, in the spirit of a creative-cloud launcher.

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use eframe::egui::{self, Align, Color32, FontId, Layout, RichText, Stroke, TextStyle, Vec2};

use crate::api::LAUNCHER_VERSION;
use crate::endpoint::Environment;
use crate::error::Result;
use crate::model::{Model, Page, Phase, Primary};
use crate::notify;
use crate::paths::Paths;
use crate::state::{self, Settings};
use crate::worker::{CHECK_EVERY, Event, Job, Worker};

pub struct Startup {
    pub paths: Paths,
    pub environment: Environment,
    pub base: String,
    pub device_label: String,
}

pub fn run(startup: Startup) -> Result<()> {
    let paths = startup.paths.clone();
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1180.0, 760.0]).with_min_inner_size([960.0, 640.0]).with_title("Smith Creative Cloud"),
        persist_window: true,
        ..Default::default()
    };
    let _ = std::fs::create_dir_all(paths.ui_dir());
    options.persistence_path = Some(paths.ui_dir());
    eframe::run_native("Smith Creative Cloud", options, Box::new(move |cc| Ok(Box::new(CloudApp::new(cc, startup)))))
        .map_err(|err| crate::error::Error::msg(err.to_string()))
}

struct CloudApp {
    model: Model,
    jobs: Sender<Job>,
    worker: Worker,
    paths: Paths,
    textures: HashMap<String, egui::TextureHandle>,
    next_catalog: Instant,
    device_label: String,
}

impl CloudApp {
    fn new(cc: &eframe::CreationContext<'_>, startup: Startup) -> Self {
        apply_theme(&cc.egui_ctx);
        let installs = state::InstallDb::load(&startup.paths).unwrap_or_default();
        let notified = state::Notified::load(&startup.paths).unwrap_or_default();
        let model = Model::new(startup.environment, installs, notified);
        let device = crate::device::Device::load_or_create(&startup.paths).unwrap_or(crate::device::Device {
            id: "00000000-0000-4000-8000-000000000000".into(),
            name: "PC".into(),
            os: "unknown".into(),
        });
        let worker = Worker::spawn(startup.paths.clone(), device, startup.environment, startup.base.clone()).unwrap_or_else(|_| Worker::disconnected());
        let jobs = worker.tx.clone();
        let _ = jobs.send(Job::Restore);
        Self {
            model,
            jobs,
            worker,
            paths: startup.paths,
            textures: HashMap::new(),
            next_catalog: Instant::now() + CHECK_EVERY,
            device_label: startup.device_label,
        }
    }

    fn apply(&mut self, ctx: &egui::Context, event: Event) {
        match event {
            Event::Restoring => {
                self.model.phase = Phase::Boot;
                self.model.status_line = "Checking your sign-in…".into();
            }
            Event::SignedOut { message } => {
                self.model.phase = Phase::SignedOut;
                self.model.user = None;
                self.model.polling = false;
                if !message.is_empty() {
                    self.model.status_line = message;
                } else {
                    self.model.status_line = "Sign in with Google to install and update the Smith Suite.".into();
                }
            }
            Event::Ready { user } => {
                self.model.phase = Phase::Ready;
                self.model.user = Some(user);
                self.model.polling = false;
                self.model.banner = None;
                self.next_catalog = Instant::now() + CHECK_EVERY;
            }
            Event::Health(health) => self.model.health = Some(health),
            Event::Catalog(catalog) => {
                let update = catalog.launcher.clone().filter(|rel| crate::version::is_newer(&rel.version, LAUNCHER_VERSION));
                self.model.self_update = update;
                self.model.catalog = Some(catalog);
                self.model.catalog_message = None;
                self.model.record_notices();
                self.flush_notices();
            }
            Event::CatalogUnavailable { message } => {
                self.model.catalog_message = Some(message);
                self.next_catalog = Instant::now() + Duration::from_secs(60);
            }
            Event::Banner(text) => self.model.banner = Some(text),
            Event::Status(text) => self.model.status_line = text,
            Event::Polling(on) => self.model.polling = on,
            Event::Progress { app_id, fraction } => {
                if fraction < 0.0 {
                    self.model.busy.remove(&app_id);
                } else {
                    self.model.busy.insert(app_id, fraction.clamp(0.0, 1.0));
                }
            }
            Event::Installs(installs) => {
                self.model.installs = installs;
                self.model.record_notices();
                self.flush_notices();
            }
            Event::SelfUpdate(update) => self.model.self_update = update,
            Event::Admin(stats) => self.model.admin = Some(stats),
            Event::Users(users) => self.model.users = users,
            Event::Exit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }

    fn flush_notices(&mut self) {
        let notices = self.model.take_notices();
        if notices.is_empty() {
            return;
        }
        for notice in &notices {
            notify::notify("Smith Creative Cloud", notice);
        }
        if let Err(err) = self.model.notified().save(&self.paths) {
            self.model.banner = Some(err.to_string());
        }
    }

    fn switch_environment(&self, ctx: &egui::Context, environment: Environment) {
        if environment == self.model.environment {
            return;
        }
        let settings = Settings { environment };
        if let Err(err) = settings.save(&self.paths) {
            let _ = err;
            return;
        }
        if let Ok(exe) = std::env::current_exe() {
            let _ = std::process::Command::new(exe).arg("--env").arg(environment.as_str()).spawn();
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for CloudApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        apply_theme(&ctx);
        while let Ok(event) = self.worker.rx.try_recv() {
            self.apply(&ctx, event);
        }
        if self.model.phase == Phase::Ready && Instant::now() >= self.next_catalog {
            let _ = self.jobs.send(Job::RefreshCatalog);
            let _ = self.jobs.send(Job::Heartbeat);
            self.next_catalog = Instant::now() + CHECK_EVERY;
        }
        let delay = if self.model.polling || !self.model.busy.is_empty() { Duration::from_millis(100) } else { Duration::from_secs(1) };
        ctx.request_repaint_after(delay);

        if self.model.phase != Phase::Ready {
            sign_in(ui, self);
        } else {
            shell(ui, self);
        }
    }
}

fn sign_in(ui: &mut egui::Ui, app: &mut CloudApp) {
    let ctx = ui.ctx().clone();
    egui::CentralPanel::default().show(ui, |ui| {
        ui.add_space(72.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("SC").size(28.0).strong().color(Color32::WHITE));
            ui.add_space(8.0);
            ui.label(RichText::new("Smith Creative Cloud").size(28.0).strong());
            ui.label(RichText::new("The Smith Suite, in one place.").color(muted()));
            ui.add_space(18.0);
            ui.allocate_ui_with_layout(Vec2::new(440.0, 0.0), Layout::top_down(Align::Min), |ui| {
                card_frame().show(ui, |ui| {
                    ui.label("Server");
                    ui.horizontal(|ui| {
                        for environment in [Environment::Prod, Environment::Staging, Environment::Mock] {
                            let selected = app.model.environment == environment;
                            let text = environment.label();
                            let response =
                                if selected { ui.add(egui::Button::new(RichText::new(text).color(Color32::WHITE)).fill(accent())) } else { ui.button(text) };
                            if response.clicked() {
                                app.switch_environment(&ctx, environment);
                            }
                        }
                    });
                    ui.add_space(8.0);
                    ui.label(RichText::new(app.model.status_line.as_str()).color(muted()));
                    ui.add_space(12.0);
                    if app.model.polling {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label("Waiting for the browser…");
                        });
                        if ui.button("Cancel").clicked() {
                            let _ = app.jobs.send(Job::Cancel);
                        }
                    } else if app.model.show_dev_sign_in() {
                        if ui
                            .add(
                                egui::Button::new(RichText::new("Sign in with a test user").color(Color32::WHITE))
                                    .fill(accent())
                                    .min_size(Vec2::new(ui.available_width(), 36.0)),
                            )
                            .clicked()
                        {
                            let _ = app.jobs.send(Job::SignIn { dev_user: true });
                        }
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("Google sign-in is not configured on staging yet. This test user exists only on the staging server.")
                                .size(12.0)
                                .color(muted()),
                        );
                    } else if ui
                        .add(
                            egui::Button::new(RichText::new("Continue with Google").color(Color32::WHITE))
                                .fill(accent())
                                .min_size(Vec2::new(ui.available_width(), 36.0)),
                        )
                        .clicked()
                    {
                        let _ = app.jobs.send(Job::SignIn { dev_user: false });
                    }
                    if let Some(banner) = &app.model.banner {
                        ui.add_space(8.0);
                        ui.label(RichText::new(banner).color(warn()));
                    }
                });
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Your Google password stays in the browser. On Windows the refresh token is kept in Credential Manager.")
                        .size(12.0)
                        .color(muted()),
                );
                ui.label(RichText::new("Smith Creative Cloud is part of Smith Suite and is not affiliated with Adobe.").size(11.0).color(muted()));
            });
        });
    });
}

fn shell(ui: &mut egui::Ui, app: &mut CloudApp) {
    egui::Panel::left("nav").exact_size(228.0).resizable(false).show(ui, |ui| {
        ui.add_space(16.0);
        ui.label(RichText::new("Smith").strong().size(20.0));
        ui.label(RichText::new("Creative Cloud").color(muted()));
        ui.add_space(18.0);
        nav_button(ui, "All apps", app.model.page == Page::Apps, || app.model.page = Page::Apps);
        let updates = format!("Updates  {}", app.model.update_count());
        nav_button(ui, &updates, app.model.page == Page::Updates, || app.model.page = Page::Updates);
        if app.model.user.as_ref().is_some_and(|user| user.is_admin) {
            let open = app.model.page == Page::Admin;
            if nav_button(ui, "Admin", open, || app.model.page = Page::Admin) && app.model.admin.is_none() {
                let _ = app.jobs.send(Job::LoadAdmin);
            }
        }
        ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
            if ui.button("Sign out").clicked() {
                let _ = app.jobs.send(Job::SignOut);
            }
            ui.label(RichText::new(app.model.environment.label()).size(12.0).color(accent()));
            ui.label(RichText::new(format!("v{LAUNCHER_VERSION}")).size(12.0).color(muted()));
            if let Some(user) = &app.model.user {
                ui.label(RichText::new(user.name.as_str()).strong());
                ui.label(RichText::new(user.email.as_str()).size(12.0).color(muted()));
            }
            ui.add_space(8.0);
        });
    });

    egui::CentralPanel::default().show(ui, |ui| {
        ui.horizontal(|ui| {
            let title = match app.model.page {
                Page::Apps => "All apps",
                Page::Updates => "Updates",
                Page::Admin => "Admin",
            };
            ui.heading(title);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if app.model.update_count() > 0 && ui.button("Update all").clicked() {
                    for card in app.model.cards() {
                        if card.update_available {
                            let _ = app.jobs.send(Job::Install { app_id: card.id });
                        }
                    }
                }
                ui.add(egui::TextEdit::singleline(&mut app.model.search).hint_text("Search").desired_width(220.0));
            });
        });
        if let Some(message) = &app.model.catalog_message {
            ui.add_space(6.0);
            banner(ui, message, warn());
            if ui.button("Check again").clicked() {
                let _ = app.jobs.send(Job::RefreshCatalog);
            }
        }
        if let Some(release) = app.model.self_update.clone() {
            ui.add_space(6.0);
            let text = if release.mandatory {
                format!("Smith Creative Cloud {} is required before you can install apps.", release.version)
            } else {
                format!("Smith Creative Cloud {} is ready.", release.version)
            };
            banner(ui, &text, accent());
            if ui.button("Update and restart").clicked() {
                let _ = app.jobs.send(Job::SelfUpdate);
            }
        }
        if let Some(banner_text) = &app.model.banner {
            ui.add_space(6.0);
            banner(ui, banner_text, warn());
        }
        ui.add_space(8.0);
        if app.model.page == Page::Admin {
            admin_page(ui, app);
        } else {
            app_grid(ui, app);
        }
        ui.add_space(6.0);
        ui.label(RichText::new(app.device_label.as_str()).size(11.0).color(muted()));
    });
}

fn app_grid(ui: &mut egui::Ui, app: &mut CloudApp) {
    let cards = app.model.visible_cards();
    if cards.is_empty() {
        ui.label(RichText::new("Nothing to show.").color(muted()));
        return;
    }
    egui::ScrollArea::vertical().show(ui, |ui| {
        let width = ui.available_width();
        let cols = ((width / 300.0).floor() as usize).max(1);
        let mut index = 0;
        while index < cards.len() {
            ui.columns(cols, |columns| {
                for (col, column) in columns.iter_mut().enumerate() {
                    if let Some(card) = cards.get(index + col) {
                        draw_card(column, card, app);
                    }
                }
            });
            index += cols;
            ui.add_space(8.0);
        }
    });
}

fn draw_card(ui: &mut egui::Ui, card: &crate::model::Card, app: &mut CloudApp) {
    card_frame().show(ui, |ui| {
        ui.set_min_height(168.0);
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(46.0), egui::Sense::hover());
            let color = Color32::from_rgb(card.color[0], card.color[1], card.color[2]);
            ui.painter().rect_filled(rect, 10.0, color);
            if let Some(texture) = app.textures.get(&card.id) {
                ui.painter().image(texture.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
            } else {
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, &card.monogram, FontId::proportional(16.0), Color32::WHITE);
            }
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&card.name).strong().size(16.0));
                    if card.update_available {
                        ui.label(RichText::new(" UPDATE").size(11.0).color(warn()).strong());
                    }
                });
                ui.label(RichText::new(&card.tagline).size(12.0).color(muted()));
            });
        });
        ui.add_space(6.0);
        let version = match (&card.installed, &card.latest) {
            (Some(have), Some(latest)) if have != latest => format!("Installed {have}  ·  Latest {latest}"),
            (Some(have), _) => format!("Installed {have}"),
            (None, Some(latest)) => format!("Version {latest}"),
            (None, None) => "Not installed".to_string(),
        };
        ui.label(RichText::new(version).size(12.0).color(muted()));
        if let Some(fraction) = app.model.busy.get(&card.id).copied() {
            ui.add(egui::ProgressBar::new(fraction).show_percentage().desired_width(ui.available_width()));
        }
        if let Some(reason) = &card.reason {
            ui.label(RichText::new(reason).size(12.0).color(muted()));
        }
        ui.horizontal(|ui| {
            let label = match card.primary {
                Primary::Install => "Install",
                Primary::Update => "Update",
                Primary::Open => "Open",
                Primary::None => "",
            };
            if card.primary != Primary::None && ui.add(egui::Button::new(RichText::new(label).color(Color32::WHITE)).fill(accent())).clicked() {
                match card.primary {
                    Primary::Install | Primary::Update => {
                        let _ = app.jobs.send(Job::Install { app_id: card.id.clone() });
                    }
                    Primary::Open => {
                        let _ = app.jobs.send(Job::Open { app_id: card.id.clone() });
                    }
                    Primary::None => {}
                }
            }
            if card.can_open && card.primary != Primary::Open && ui.button("Open").clicked() {
                let _ = app.jobs.send(Job::Open { app_id: card.id.clone() });
            }
            if card.can_uninstall && ui.button(RichText::new("Uninstall").color(danger())).clicked() {
                let _ = app.jobs.send(Job::Uninstall { app_id: card.id.clone() });
            }
        });
        if !card.notes.is_empty() && ui.small_button("Release notes").clicked() {
            if app.model.notes_for.as_deref() == Some(card.id.as_str()) {
                app.model.notes_for = None;
            } else {
                app.model.notes_for = Some(card.id.clone());
            }
        }
        if app.model.notes_for.as_deref() == Some(card.id.as_str()) {
            ui.add_space(4.0);
            ui.label(RichText::new(&card.notes).size(12.0).color(muted()));
        }
    });
}

fn admin_page(ui: &mut egui::Ui, app: &CloudApp) {
    let Some(stats) = &app.model.admin else {
        ui.label("Loading admin stats…");
        return;
    };
    ui.horizontal(|ui| {
        stat(ui, "Users", stats.users_total);
        stat(ui, "Active 30d", stats.users_active_30d);
        stat(ui, "Devices", stats.devices_total);
    });
    ui.add_space(8.0);
    for row in &stats.installs_by_app {
        ui.label(format!(
            "{}  installed {}  all-time {}  latest {} ({})",
            row.app_id, row.installed_now, row.installs_total, row.latest_version, row.on_latest
        ));
    }
    ui.add_space(8.0);
    for user in &app.model.users {
        ui.label(format!("{}  {}  devices {}", user.email, user.name, user.devices));
    }
}

fn stat(ui: &mut egui::Ui, label: &str, value: u64) {
    card_frame().show(ui, |ui| {
        ui.set_min_width(140.0);
        ui.label(RichText::new(value.to_string()).size(24.0).strong());
        ui.label(RichText::new(label).color(muted()));
    });
}

fn nav_button(ui: &mut egui::Ui, label: &str, selected: bool, mut select: impl FnMut()) -> bool {
    let response = if selected {
        ui.add(egui::Button::new(RichText::new(label).color(Color32::WHITE)).fill(accent()).min_size(Vec2::new(ui.available_width(), 32.0)))
    } else {
        ui.add(egui::Button::new(label).min_size(Vec2::new(ui.available_width(), 32.0)))
    };
    if response.clicked() {
        select();
    }
    response.clicked()
}

fn banner(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::new().fill(Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 40)).corner_radius(8).inner_margin(10).show(ui, |ui| {
        ui.label(RichText::new(text).color(Color32::WHITE));
    });
}

fn card_frame() -> egui::Frame {
    egui::Frame::new().fill(Color32::from_rgb(34, 38, 51)).stroke(Stroke::new(1.0, Color32::from_rgb(52, 58, 76))).corner_radius(12).inner_margin(14)
}

fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = Color32::from_rgb(18, 20, 26);
    visuals.panel_fill = Color32::from_rgb(18, 20, 26);
    visuals.extreme_bg_color = Color32::from_rgb(12, 13, 18);
    visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(244, 246, 251);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(42, 48, 64);
    visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(244, 246, 251);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(58, 78, 120);
    visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;
    visuals.widgets.active.bg_fill = accent();
    visuals.widgets.active.fg_stroke.color = Color32::WHITE;
    visuals.selection.bg_fill = Color32::from_rgb(76, 141, 255);
    visuals.hyperlink_color = Color32::from_rgb(158, 190, 255);
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        style.spacing.item_spacing = Vec2::new(10.0, 8.0);
        style.spacing.button_padding = Vec2::new(12.0, 6.0);
        style.text_styles.insert(TextStyle::Heading, FontId::proportional(22.0));
        style.text_styles.insert(TextStyle::Body, FontId::proportional(14.5));
        style.text_styles.insert(TextStyle::Button, FontId::proportional(14.0));
        style.text_styles.insert(TextStyle::Small, FontId::proportional(12.0));
    });
}

fn accent() -> Color32 {
    Color32::from_rgb(76, 141, 255)
}
fn muted() -> Color32 {
    Color32::from_rgb(154, 163, 184)
}
fn warn() -> Color32 {
    Color32::from_rgb(255, 176, 32)
}
fn danger() -> Color32 {
    Color32::from_rgb(255, 107, 122)
}
