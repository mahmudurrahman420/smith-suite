//! Framesmith's phone and tablet layout.
//!
//! The desktop arranges panels in a docking layout made for a mouse and a big screen. On a phone
//! the same engine and the same panels are arranged for a thumb instead, in portrait or landscape:
//!
//! ```text
//!  portrait                         landscape
//!  ┌──────────────────────┐         ┌───┬──────────────────────────┬───────────┐
//!  │ ☰  project      Export│         │ ▣ │ ☰  project        Export │           │
//!  ├──────────────────────┤         │ ▣ ├──────────────────────────┤   sheet   │
//!  │       preview        │         │ ▣ │         preview          │ (a panel, │
//!  ├──────────────────────┤         │ ▣ ├──────────────────────────┤  tools…)  │
//!  │ tool  00:00  ◀ ▶ ▶▶  │         │ ▣ │ tool  00:00  ◀ ▶ ▶▶       │           │
//!  ├──────────────────────┤         │ ▣ ├──────────────────────────┤           │
//!  │  timeline  / sheet   │         │ ▣ │         timeline         │           │
//!  ├──────────────────────┤         └───┴──────────────────────────┴───────────┘
//!  │ ▣  ▣  ▣  ▣  ▣  ▣  ▣ │          tool rail                        side sheet
//!  └──────────────────────┘
//! ```
//!
//! The toolbar changes with the selection: with nothing selected it adds things (media, audio,
//! text, effects, transitions, colour, canvas…); with clips selected it edits them (split, delete,
//! speed, volume, adjust, colour…). Everything else the desktop can do is one tap away: every
//! panel opens in a sheet ("Panels"), and the full menu tree is behind ☰. Touch gestures are
//! translated for the panels in [`touch`]; file dialogs are answered by the in-app [`picker`].

pub mod picker;
pub mod touch;

use egui::{Align2, Color32, Key, Rect, Sense, Stroke, StrokeKind, pos2, vec2};
use filmcraft_time::{TimeDisplay, format_time};
use serde_json::{Value, json};

use crate::FilmcraftApp;
use crate::dock::PanelKind;
use crate::icons::{self, Icon};
use crate::native_dialogs::FileDialog;
use crate::state::Tool;
use crate::theme::Tokens;

/// Screen edges covered by the system (status bar, display cutout, navigation bar), in points.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Insets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// What a sheet shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    /// Any desktop panel.
    Panel(PanelKind),
    /// The project's media, with Add from phone and Add to timeline.
    Media,
    /// Music, sounds, voice-over and the audio panels.
    Audio,
    /// Titles, shapes and captions.
    Text,
    /// Transitions at the cut under the playhead.
    Transitions,
    /// Frame size and rate presets.
    Canvas,
    /// Every panel, to open one.
    Panels,
    /// The menu bar's commands.
    Menu,
}

/// State of the phone layout ([`FilmcraftApp::mobile`]).
#[derive(Default)]
pub struct MobileShell {
    pub touch: touch::Touch,
    pub picker: picker::Picker,
    /// System bars to stay clear of (the host updates them).
    pub insets: Insets,
    pub sheet: Option<Sheet>,
    /// A portrait sheet covers the preview too.
    pub sheet_tall: bool,
    /// The submenu the Menu sheet shows (menu path).
    menu_path: Vec<String>,
    /// The preview fills the screen.
    pub fullscreen: bool,
    /// The tool picker is open.
    tools_open: bool,
    /// The soft keyboard as last asked of the host.
    keyboard: bool,
    /// Export jobs already announced (and handed to the media scanner).
    announced: Vec<u64>,
    /// Draw the next Program monitor without its desktop controls (the layout has its own).
    pub(crate) compact_monitor: bool,
}


/// Timeline track headers on a phone: narrow, the timeline gets the width.
const HEADER_W: f32 = 84.0;
/// A finger-sized button.
const TOUCH: f32 = 44.0;

impl FilmcraftApp {
    /// Switch to the phone layout: touch gestures, the in-app file picker, no desktop chrome.
    pub fn enable_mobile(&mut self) {
        let shell = MobileShell::default();
        self.hooks.run_dialog = Some(picker::hook(shell.picker.queue.clone()));
        self.mobile = Some(shell);
        self.ui.show_header = false;
        self.ui.show_status_bar = false;
        self.ui.show_menu_bar = false;
    }

    /// Whether the phone layout is on.
    pub fn is_mobile(&self) -> bool {
        self.mobile.is_some()
    }
}

/// Run a command (menu or engine) and report a failure.
fn run(app: &mut FilmcraftApp, ctx: &egui::Context, id: &str, params: Value) {
    if let Err(e) = crate::menus::invoke(app, ctx, id, params) {
        app.toast = Some((e, ctx.input(|i| i.time)));
    }
}

/// A rounded text button; `primary` fills it with the accent.
pub(crate) fn text_button(ui: &mut egui::Ui, r: Rect, label: &str, primary: bool, t: &Tokens, id: &str) -> egui::Response {
    let resp = ui.interact(r, egui::Id::new(("mobile-btn", id)), Sense::click());
    let down = resp.is_pointer_button_down_on();
    let bg = if primary {
        if down { t.accent_hover } else { t.accent }
    } else if down {
        t.pressed
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(r, r.height() / 2.0, bg);
    let fg = if primary { Color32::WHITE } else { t.text };
    ui.painter().text(r.center(), Align2::CENTER_CENTER, label, Tokens::semibold(15.0), fg);
    resp
}

/// A square icon button.
fn icon_button(ui: &mut egui::Ui, r: Rect, icon: Icon, on: bool, enabled: bool, t: &Tokens, id: &str) -> egui::Response {
    let sense = if enabled { Sense::click() } else { Sense::hover() };
    let resp = ui.interact(r, egui::Id::new(("mobile-icon", id)), sense);
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(r.shrink(2.0), 10.0, t.pressed);
    } else if on {
        ui.painter().rect_filled(r.shrink(2.0), 10.0, t.accent.gamma_multiply(0.3));
    }
    let c = if !enabled {
        t.text_faint
    } else if on {
        t.focus
    } else {
        t.icon_active
    };
    let s = (r.height() * 0.46).min(24.0);
    icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(s, s)), icon, c);
    resp
}

/// A toolbar item: icon above a short label.
fn tool_item(ui: &mut egui::Ui, r: Rect, icon: Icon, label: &str, on: bool, t: &Tokens, id: &str) -> egui::Response {
    let resp = ui.interact(r, egui::Id::new(("mobile-tool", id)), Sense::click());
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(r.shrink(3.0), 12.0, t.pressed);
    } else if on {
        ui.painter().rect_filled(r.shrink(3.0), 12.0, t.accent.gamma_multiply(0.28));
    }
    let c = if on { t.focus } else { t.icon_active };
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.center().x, r.min.y + r.height() * 0.38), vec2(22.0, 22.0)), icon, c);
    let mut job = egui::text::LayoutJob::simple_singleline(crate::i18n::t(label).to_string(), Tokens::ui(11.0), if on { t.focus } else { t.text_dim });
    job.wrap.max_width = r.width() - 4.0;
    job.wrap.max_rows = 1;
    job.halign = egui::Align::Center;
    let g = ui.painter().layout_job(job);
    ui.painter().galley(pos2(r.center().x, r.max.y - r.height() * 0.3 - g.size().y / 2.0), g, t.text_dim);
    resp
}

/// What a toolbar item does.
#[derive(Clone, Copy)]
enum Act {
    Open(Sheet),
    Command(&'static str),
    Deselect,
}

/// The toolbar with nothing selected: things to add.
const MAIN_TOOLS: [(&str, Icon, Act); 11] = [
    ("Media", Icon::Plus, Act::Open(Sheet::Media)),
    ("Audio", Icon::Audio, Act::Open(Sheet::Audio)),
    ("Text", Icon::Type, Act::Open(Sheet::Text)),
    ("Effects", Icon::Fx, Act::Open(Sheet::Panel(PanelKind::Effects))),
    ("Transitions", Icon::Sparkle, Act::Open(Sheet::Transitions)),
    ("Color", Icon::Eyedropper, Act::Open(Sheet::Panel(PanelKind::LumetriColor))),
    ("Canvas", Icon::Square, Act::Open(Sheet::Canvas)),
    ("Captions", Icon::Captions, Act::Open(Sheet::Panel(PanelKind::Text))),
    ("Marker", Icon::Marker, Act::Command("markers.add")),
    ("Mixer", Icon::Speaker, Act::Open(Sheet::Panel(PanelKind::AudioTrackMixer))),
    ("Panels", Icon::Grid, Act::Open(Sheet::Panels)),
];

/// The toolbar with clips selected: edit them.
const CLIP_TOOLS: [(&str, Icon, Act); 16] = [
    ("Done", Icon::ChevronLeft, Act::Deselect),
    ("Split", Icon::Razor, Act::Command("sequence.addEdit")),
    ("Delete", Icon::Trash, Act::Command("edit.rippleDelete")),
    ("Duplicate", Icon::NewItem, Act::Command("edit.duplicate")),
    ("Speed", Icon::Stopwatch, Act::Command("clip.speedDuration")),
    ("Volume", Icon::Speaker, Act::Command("clip.audioGain")),
    ("Adjust", Icon::Adjust, Act::Open(Sheet::Panel(PanelKind::EffectControls))),
    ("Color", Icon::Eyedropper, Act::Open(Sheet::Panel(PanelKind::LumetriColor))),
    ("Effects", Icon::Fx, Act::Open(Sheet::Panel(PanelKind::Effects))),
    ("Transform", Icon::Selection, Act::Open(Sheet::Panel(PanelKind::Properties))),
    ("Freeze", Icon::Pause, Act::Command("clip.frameHold")),
    ("Extract audio", Icon::Audio, Act::Command("clip.extractAudio")),
    ("Fill frame", Icon::Fullscreen, Act::Command("clip.fillFrame")),
    ("Copy", Icon::Insert, Act::Command("edit.copy")),
    ("Disable", Icon::EyeOff, Act::Command("clip.enable")),
    ("Nest", Icon::Nest, Act::Command("clip.nest")),
];

fn do_act(app: &mut FilmcraftApp, ctx: &egui::Context, act: Act) {
    match act {
        Act::Open(sheet) => {
            if let Some(m) = app.mobile.as_mut() {
                // the same tool again closes its sheet
                m.sheet = if m.sheet == Some(sheet) { None } else { Some(sheet) };
                m.menu_path.clear();
            }
            if let Act::Open(Sheet::Panel(p)) = act {
                app.ui.focused = p;
            }
        }
        Act::Command(id) => run(app, ctx, id, json!({})),
        Act::Deselect => {
            run(app, ctx, "edit.deselectAll", json!({}));
            if let Some(m) = app.mobile.as_mut() {
                m.sheet = None;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MediaKind {
    Any,
    Audio,
}

/// Pick files from the phone, import them, and put them on the timeline at the playhead (a new
/// sequence shaped like the first clip when there is none yet).
fn add_media(app: &mut FilmcraftApp, kind: MediaKind) {
    let exts: Vec<&str> = match kind {
        MediaKind::Audio => filmcraft_media::AUDIO_EXTENSIONS.to_vec(),
        MediaKind::Any => filmcraft_media::VIDEO_EXTENSIONS.iter().chain(filmcraft_media::AUDIO_EXTENSIONS).chain(filmcraft_media::STILL_EXTENSIONS).copied().collect(),
    };
    let r = app.pick(FileDialog::open_files(&exts), |app, paths| {
        if paths.is_empty() {
            return Ok(Value::Null);
        }
        let bin = app.import_bin().0;
        let r = app.session.execute("file.import", json!({"paths": paths, "bin": bin})).map_err(|e| e.to_string())?;
        let items: Vec<u64> = r.get("items").and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_u64).collect()).unwrap_or_default();
        if let Some(errs) = r.get("errors").and_then(Value::as_array)
            && !errs.is_empty()
        {
            app.ui.status = errs.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("; ");
        }
        place_on_timeline(app, &items)?;
        if let Some(m) = app.mobile.as_mut() {
            m.sheet = None;
        }
        Ok(r)
    });
    if let Err(e) = r {
        app.ui.status = e;
    }
}

/// Insert project items at the playhead, or start a sequence from them.
fn place_on_timeline(app: &mut FilmcraftApp, items: &[u64]) -> Result<(), String> {
    if items.is_empty() {
        return Ok(());
    }
    if app.session.state.active_sequence.is_none() {
        let name = app.session.project.item(filmcraft_project::ItemId(items[0])).map(|i| i.name.clone()).unwrap_or_else(|| "Sequence 01".into());
        app.session.execute("file.newSequence", json!({"name": name, "fromItems": items})).map_err(|e| e.to_string())?;
        return Ok(());
    }
    app.session.execute("timeline.place", json!({"items": items, "insert": true})).map(|_| ()).map_err(|e| e.to_string())
}

/// Draw the phone layout into `full` (the whole window).
pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, full: Rect) {
    let ctx = ui.ctx().clone();
    let t = app.tokens;
    ui.painter().rect_filled(full, 0.0, t.app_bg);
    if app.ui.timeline.header_w > HEADER_W {
        app.ui.timeline.header_w = HEADER_W;
    }
    let Some(insets) = app.mobile.as_ref().map(|m| m.insets) else { return };
    let area = Rect::from_min_max(full.min + vec2(insets.left, insets.top), full.max - vec2(insets.right, insets.bottom));
    handle_back(app, &ctx);
    if picker::show(app, &ctx, full) {
        keyboard(app, &ctx);
        return;
    }
    if app.ui.mode == crate::state::Mode::Export || app.ui.mode == crate::state::Mode::Import {
        // Export settings and Home are full-screen pages with a way back
        let bar = Rect::from_min_size(area.min, vec2(area.width(), 48.0));
        ui.painter().rect_filled(bar, 0.0, t.header_bg);
        if icon_button(ui, Rect::from_min_size(bar.min, vec2(48.0, 48.0)), Icon::ChevronLeft, false, true, &t, "page.back").clicked() {
            app.ui.mode = crate::state::Mode::Edit;
        }
        let title = if app.ui.mode == crate::state::Mode::Export { crate::i18n::t("Export") } else { crate::i18n::t("Home") };
        ui.painter().text(bar.center(), Align2::CENTER_CENTER, title, Tokens::semibold(16.0), t.text);
        let body = Rect::from_min_max(pos2(area.min.x, bar.max.y), area.max);
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt("mobile-page"));
        child.set_clip_rect(body);
        if app.ui.mode == crate::state::Mode::Export {
            crate::panels::export_mode::show(app, &mut child, body);
        } else {
            crate::panels::import_mode::show(app, &mut child, body);
        }
        overlays(app, ui, &ctx, area);
        keyboard(app, &ctx);
        return;
    }
    if app.mobile.as_ref().is_some_and(|m| m.fullscreen) {
        fullscreen(app, ui, full);
        keyboard(app, &ctx);
        return;
    }
    let portrait = area.height() >= area.width();
    let selecting = !app.session.state.selection.is_empty();
    let sheet = app.mobile.as_ref().and_then(|m| m.sheet);
    let tall = app.mobile.as_ref().is_some_and(|m| m.sheet_tall);
    if portrait {
        let top = Rect::from_min_size(area.min, vec2(area.width(), 52.0));
        let toolbar = Rect::from_min_max(pos2(area.min.x, area.max.y - 74.0), area.max);
        let preview_h = (area.height() * 0.34).clamp(160.0, area.width() * 1.1);
        let preview = Rect::from_min_size(pos2(area.min.x, top.max.y), vec2(area.width(), preview_h));
        let transport = Rect::from_min_size(pos2(area.min.x, preview.max.y), vec2(area.width(), 50.0));
        let middle = Rect::from_min_max(pos2(area.min.x, transport.max.y), pos2(area.max.x, toolbar.min.y));
        top_bar(app, ui, &ctx, top);
        program(app, ui, preview);
        transport_row(app, ui, &ctx, transport);
        timeline(app, ui, middle);
        toolbar_row(app, ui, &ctx, toolbar, selecting);
        if let Some(s) = sheet {
            let r = if tall { Rect::from_min_max(pos2(area.min.x, top.max.y), middle.max) } else { middle };
            sheet_ui(app, ui, &ctx, r, s, true);
        }
    } else {
        let rail = Rect::from_min_max(area.min, pos2(area.min.x + 78.0, area.max.y));
        let sheet_w = if sheet.is_some() { ((area.width() - rail.width()) * 0.42).clamp(300.0, 460.0) } else { 0.0 };
        let center = Rect::from_min_max(pos2(rail.max.x, area.min.y), pos2(area.max.x - sheet_w, area.max.y));
        let top = Rect::from_min_size(center.min, vec2(center.width(), 46.0));
        let preview_h = ((center.height() - 46.0 - 44.0) * 0.46).max(110.0);
        let preview = Rect::from_min_size(pos2(center.min.x, top.max.y), vec2(center.width(), preview_h));
        let transport = Rect::from_min_size(pos2(center.min.x, preview.max.y), vec2(center.width(), 44.0));
        let tl = Rect::from_min_max(pos2(center.min.x, transport.max.y), center.max);
        rail_column(app, ui, &ctx, rail, selecting);
        top_bar(app, ui, &ctx, top);
        program(app, ui, preview);
        transport_row(app, ui, &ctx, transport);
        timeline(app, ui, tl);
        if let Some(s) = sheet {
            let r = Rect::from_min_max(pos2(center.max.x, area.min.y), area.max);
            sheet_ui(app, ui, &ctx, r, s, false);
        }
    }
    tools_popup(app, &ctx, area);
    overlays(app, ui, &ctx, area);
    announce_exports(app, &ctx);
    keyboard(app, &ctx);
    if app.mobile.as_ref().is_some_and(|m| m.touch.waiting()) {
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}

/// The Android back button (Escape on a keyboard) closes the topmost thing; with nothing open it
/// leaves for the home screen.
fn handle_back(app: &mut FilmcraftApp, ctx: &egui::Context) {
    let back = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::BrowserBack));
    if !back {
        return;
    }
    let Some(m) = app.mobile.as_mut() else { return };
    if m.picker.open() {
        m.picker.cancel();
    } else if app.ui.export.quick_open {
        app.ui.export.quick_open = false;
    } else if m.tools_open {
        m.tools_open = false;
    } else if m.sheet == Some(Sheet::Menu) && !m.menu_path.is_empty() {
        m.menu_path.pop();
    } else if m.sheet.is_some() {
        m.sheet = None;
        m.sheet_tall = false;
    } else if m.fullscreen {
        m.fullscreen = false;
    } else if app.ui.mode != crate::state::Mode::Edit {
        app.ui.mode = crate::state::Mode::Edit;
    } else if !app.session.state.selection.is_empty() {
        run(app, ctx, "edit.deselectAll", json!({}));
    } else if let Some(f) = app.hooks.leave.as_mut() {
        f();
    }
}

/// Show the soft keyboard while a text field wants typing.
fn keyboard(app: &mut FilmcraftApp, ctx: &egui::Context) {
    let wants = ctx.egui_wants_keyboard_input();
    let Some(m) = app.mobile.as_mut() else { return };
    if m.keyboard != wants {
        m.keyboard = wants;
        if let Some(f) = app.hooks.soft_keyboard.as_mut() {
            f(wants);
        }
    }
}

fn top_bar(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, r: Rect) {
    let t = app.tokens;
    ui.painter().rect_filled(r, 0.0, t.header_bg);
    let menu = Rect::from_min_size(r.min, vec2(r.height(), r.height()));
    let menu_open = app.mobile.as_ref().is_some_and(|m| m.sheet == Some(Sheet::Menu));
    if icon_button(ui, menu, Icon::Hamburger, menu_open, true, &t, "menu").clicked() {
        do_act(app, ctx, Act::Open(Sheet::Menu));
    }
    app.auto.add("mobile.menu", menu, "Menu");
    // Export on the right, then redo and undo
    let export = Rect::from_min_size(pos2(r.max.x - 96.0, r.center().y - 18.0), vec2(88.0, 36.0));
    if text_button(ui, export, crate::i18n::t("Export"), true, &t, "export").clicked() {
        if app.session.state.active_sequence.is_some() {
            app.ui.export.quick_open = !app.ui.export.quick_open;
            ctx.data_mut(|d| d.insert_temp(egui::Id::new("quick-export-toggled"), true));
        } else {
            app.toast = Some((crate::i18n::t("Add some media first").to_string(), ctx.input(|i| i.time)));
        }
    }
    app.auto.add("mobile.export", export, "Export");
    let redo = Rect::from_min_size(pos2(export.min.x - 46.0, r.center().y - 22.0), vec2(TOUCH, TOUCH));
    let undo = redo.translate(vec2(-46.0, 0.0));
    let (can_undo, can_redo) = (app.session.history.can_undo(), app.session.history.can_redo());
    if icon_button(ui, undo, Icon::Undo, false, can_undo, &t, "undo").clicked() {
        run(app, ctx, "edit.undo", json!({}));
    }
    if icon_button(ui, redo, Icon::Redo, false, can_redo, &t, "redo").clicked() {
        run(app, ctx, "edit.redo", json!({}));
    }
    app.auto.add("mobile.undo", undo, "Undo");
    app.auto.add("mobile.redo", redo, "Redo");
    // the mark, then the project's name (a dot while unsaved); tap to save
    let mark = Rect::from_center_size(pos2(menu.max.x + 14.0, r.center().y), vec2(24.0, 24.0));
    crate::brand::paint_mark(ui.painter(), mark);
    let name_r = Rect::from_min_max(pos2(mark.max.x + 8.0, r.min.y), pos2(undo.min.x - 4.0, r.max.y));
    let mut name = app.session.project.name.clone();
    if app.session.is_dirty() {
        name.push_str(" •");
    }
    let mut job = egui::text::LayoutJob::simple_singleline(name, Tokens::semibold(15.0), t.text);
    job.wrap.max_width = name_r.width().max(10.0);
    job.wrap.max_rows = 1;
    let g = ui.painter().layout_job(job);
    ui.painter().galley(pos2(name_r.min.x, r.center().y - g.size().y / 2.0), g, t.text);
    let resp = ui.interact(name_r, egui::Id::new("mobile-project-name"), Sense::click());
    if resp.clicked() {
        run(app, ctx, "file.save", json!({}));
    }
    // a running job (export, render) as a thin bar along the bottom edge
    if let Some(job) = app.session.jobs.iter().rev().find(|j| !j.progress.finished.load(std::sync::atomic::Ordering::Relaxed)).cloned() {
        let f = job.progress.fraction().clamp(0.0, 1.0);
        let bar = Rect::from_min_max(pos2(r.min.x, r.max.y - 3.0), r.max);
        ui.painter().rect_filled(bar, 0.0, t.separator);
        ui.painter().rect_filled(Rect::from_min_size(bar.min, vec2(bar.width() * f, bar.height())), 0.0, t.accent);
        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

/// The Program monitor, picture only (this layout draws its own transport).
fn program(app: &mut FilmcraftApp, ui: &mut egui::Ui, r: Rect) {
    if let Some(m) = app.mobile.as_mut() {
        m.compact_monitor = true;
    }
    panel_in(app, ui, PanelKind::Program, r);
    if let Some(m) = app.mobile.as_mut() {
        m.compact_monitor = false;
    }
}

/// Any panel in `r`, as the docking layout draws a panel body.
fn panel_in(app: &mut FilmcraftApp, ui: &mut egui::Ui, panel: PanelKind, r: Rect) {
    app.auto.add(&format!("panel.{}", panel.id()), r, panel.title());
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r).id_salt(("panel", panel.id())));
    child.set_clip_rect(r.intersect(ui.clip_rect()));
    crate::panels::show(app, &mut child, panel, r);
    // a touch in a panel gives it the keyboard focus (shortcuts and Delete act on it)
    if ui.ctx().input(|i| i.pointer.any_pressed() && i.pointer.press_origin().is_some_and(|p| r.contains(p))) {
        app.ui.focused = panel;
    }
}

fn timeline(app: &mut FilmcraftApp, ui: &mut egui::Ui, r: Rect) {
    panel_in(app, ui, PanelKind::Timeline, r);
}

fn transport_row(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, r: Rect) {
    let t = app.tokens;
    ui.painter().rect_filled(r, 0.0, t.panel_bg);
    ui.painter().line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, t.separator));
    // the edit tool, left
    let tool = app.ui.tool;
    let tr = Rect::from_min_size(pos2(r.min.x + 4.0, r.center().y - 22.0), vec2(TOUCH, TOUCH));
    let tools_open = app.mobile.as_ref().is_some_and(|m| m.tools_open);
    if icon_button(ui, tr, tool.icon(), tools_open, true, &t, "tool").clicked()
        && let Some(m) = app.mobile.as_mut()
    {
        m.tools_open = !m.tools_open;
    }
    app.auto.add("mobile.tool", tr, crate::i18n::t(tool.label()));
    // timecode / duration
    if let Some(q) = app.session.active_sequence() {
        let rate = q.settings.frame_rate;
        let df = q.settings.drop_frame;
        let tc = format_time(app.session.playhead(), rate, df, TimeDisplay::Timecode, 48000);
        let dur = format_time(q.duration(), rate, df, TimeDisplay::Timecode, 48000);
        let x = tr.max.x + 6.0;
        ui.painter().text(pos2(x, r.center().y - 8.0), Align2::LEFT_CENTER, tc, Tokens::timecode(), t.timecode);
        ui.painter().text(pos2(x, r.center().y + 9.0), Align2::LEFT_CENTER, dur, Tokens::ui(11.0), t.text_dim);
    }
    // play controls between the timecode and the full-screen button; frame steps when there is room
    let playing = app.playback.playing;
    let play_icon = if playing { Icon::Pause } else { Icon::Play };
    let all = [
        (Icon::GoToIn, "playhead.prevEdit", "prevEdit"),
        (Icon::StepBack, "playhead.stepBack", "stepBack"),
        (play_icon, "playback.toggle", "play"),
        (Icon::StepFwd, "playhead.stepForward", "stepFwd"),
        (Icon::GoToOut, "playhead.nextEdit", "nextEdit"),
    ];
    let left = tr.max.x + 104.0;
    let right = r.max.x - TOUCH - 8.0;
    let room = right - left;
    let buttons: Vec<_> = if room >= TOUCH * 5.0 { all.to_vec() } else { vec![all[0], all[2], all[4]] };
    let w = TOUCH * buttons.len() as f32;
    let cx = r.center().x.max(left + w / 2.0).min(right - w / 2.0);
    let mut x = cx - w / 2.0;
    for (icon, cmd, id) in buttons {
        let b = Rect::from_min_size(pos2(x, r.center().y - TOUCH / 2.0), vec2(TOUCH, TOUCH));
        if id == "play" {
            ui.painter().circle_filled(b.center(), 19.0, t.accent);
            let resp = ui.interact(b, egui::Id::new("mobile-play"), Sense::click());
            icons::paint(ui.painter(), Rect::from_center_size(b.center(), vec2(18.0, 18.0)), icon, Color32::WHITE);
            if resp.clicked() {
                run(app, ctx, cmd, json!({}));
            }
        } else if icon_button(ui, b, icon, false, true, &t, id).clicked() {
            run(app, ctx, cmd, json!({}));
        }
        app.auto.add(&format!("mobile.{id}"), b, id);
        x += TOUCH;
    }
    // full-screen preview, right
    let fs = Rect::from_min_size(pos2(r.max.x - TOUCH - 4.0, r.center().y - TOUCH / 2.0), vec2(TOUCH, TOUCH));
    if icon_button(ui, fs, Icon::Fullscreen, false, true, &t, "fullscreen").clicked()
        && let Some(m) = app.mobile.as_mut()
    {
        m.fullscreen = true;
    }
    app.auto.add("mobile.fullscreen", fs, "Full Screen");
}

/// The edit tools, in a grid over the timeline.
fn tools_popup(app: &mut FilmcraftApp, ctx: &egui::Context, area: Rect) {
    if !app.mobile.as_ref().is_some_and(|m| m.tools_open) {
        return;
    }
    let t = app.tokens;
    let cols = 4usize;
    let tools: Vec<Tool> = Tool::ALL.iter().copied().filter(|t| !matches!(t, Tool::Zoom | Tool::VerticalType | Tool::Remix)).collect();
    let rows = tools.len().div_ceil(cols);
    let cell = vec2(((area.width() - 32.0) / cols as f32).min(110.0), 70.0);
    let size = vec2(cell.x * cols as f32 + 16.0, cell.y * rows as f32 + 16.0);
    let rect = Rect::from_center_size(area.center(), size);
    let mut chosen = None;
    let mut close = false;
    egui::Area::new(egui::Id::new("mobile-tools")).order(egui::Order::Foreground).fixed_pos(pos2(0.0, 0.0)).show(ctx, |ui| {
        // the dimmed backdrop closes the grid
        let bg = ui.interact(ctx.content_rect(), egui::Id::new("mobile-tools-bg"), Sense::click());
        ui.painter().rect_filled(ctx.content_rect(), 0.0, Color32::from_black_alpha(120));
        close |= bg.clicked();
        ui.painter().rect_filled(rect, 16.0, t.header_bg);
        ui.painter().rect_stroke(rect, 16.0, Stroke::new(1.0, t.separator), StrokeKind::Inside);
        for (i, tool) in tools.iter().enumerate() {
            let c = Rect::from_min_size(rect.min + vec2(8.0 + (i % cols) as f32 * cell.x, 8.0 + (i / cols) as f32 * cell.y), cell);
            let label = crate::i18n::t(tool.label()).trim_end_matches(" Tool").to_string();
            if tool_item(ui, c, tool.icon(), &label, app.ui.tool == *tool, &t, &format!("tool-{i}")).clicked() {
                chosen = Some(*tool);
            }
        }
    });
    if let Some(tool) = chosen {
        app.ui.tool = tool;
        close = true;
    }
    if close && let Some(m) = app.mobile.as_mut() {
        m.tools_open = false;
    }
}

fn toolbar_items(selecting: bool) -> &'static [(&'static str, Icon, Act)] {
    if selecting { &CLIP_TOOLS } else { &MAIN_TOOLS }
}

fn act_is_on(sheet: Option<Sheet>, act: Act) -> bool {
    matches!(act, Act::Open(s) if Some(s) == sheet)
}

fn toolbar_row(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, r: Rect, selecting: bool) {
    let t = app.tokens;
    ui.painter().rect_filled(r, 0.0, t.header_bg);
    ui.painter().line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, t.separator));
    let sheet = app.mobile.as_ref().and_then(|m| m.sheet);
    let items = toolbar_items(selecting);
    let mut fired = None;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r).id_salt("mobile-toolbar"));
    egui::ScrollArea::horizontal().id_salt(("mobile-toolbar-scroll", selecting)).scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(&mut child, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(4.0);
            for (i, (label, icon, act)) in items.iter().enumerate() {
                let (c, _) = ui.allocate_exact_size(vec2(68.0, r.height()), Sense::hover());
                if tool_item(ui, c, *icon, label, act_is_on(sheet, *act), &t, &format!("bar-{selecting}-{i}")).clicked() {
                    fired = Some(*act);
                }
            }
            ui.add_space(4.0);
        });
    });
    for (i, (label, ..)) in items.iter().enumerate() {
        app.auto.add(&format!("mobile.toolbar.{i}"), Rect::from_min_size(r.min + vec2(4.0 + 68.0 * i as f32, 0.0), vec2(68.0, r.height())), label);
    }
    if let Some(a) = fired {
        do_act(app, ctx, a);
    }
}

fn rail_column(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, r: Rect, selecting: bool) {
    let t = app.tokens;
    ui.painter().rect_filled(r, 0.0, t.header_bg);
    ui.painter().line_segment([r.right_top(), r.right_bottom()], Stroke::new(1.0, t.separator));
    let sheet = app.mobile.as_ref().and_then(|m| m.sheet);
    let items = toolbar_items(selecting);
    let mut fired = None;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r).id_salt("mobile-rail"));
    egui::ScrollArea::vertical().id_salt(("mobile-rail-scroll", selecting)).scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(&mut child, |ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add_space(4.0);
            for (i, (label, icon, act)) in items.iter().enumerate() {
                let (c, _) = ui.allocate_exact_size(vec2(r.width(), 64.0), Sense::hover());
                if tool_item(ui, c, *icon, label, act_is_on(sheet, *act), &t, &format!("rail-{selecting}-{i}")).clicked() {
                    fired = Some(*act);
                }
            }
            ui.add_space(4.0);
        });
    });
    if let Some(a) = fired {
        do_act(app, ctx, a);
    }
}

fn sheet_title(s: Sheet) -> &'static str {
    match s {
        Sheet::Panel(p) => p.title(),
        Sheet::Media => "Media",
        Sheet::Audio => "Audio",
        Sheet::Text => "Text",
        Sheet::Transitions => "Transitions",
        Sheet::Canvas => "Canvas",
        Sheet::Panels => "Panels",
        Sheet::Menu => "Menu",
    }
}

/// A sheet over `r`: a header (title, grow, close) and its contents.
fn sheet_ui(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, r: Rect, s: Sheet, portrait: bool) {
    let t = app.tokens;
    // the sheet takes its area's input: nothing under it reacts
    ui.interact(r, egui::Id::new("mobile-sheet-block"), Sense::click_and_drag());
    ui.painter().rect_filled(r, 0.0, t.panel_bg);
    ui.painter().line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, t.separator));
    if !portrait {
        ui.painter().line_segment([r.left_top(), r.left_bottom()], Stroke::new(1.0, t.separator));
    }
    let head = Rect::from_min_size(r.min, vec2(r.width(), 46.0));
    ui.painter().rect_filled(head, 0.0, t.header_bg);
    let menu_path = app.mobile.as_ref().map(|m| m.menu_path.clone()).unwrap_or_default();
    let mut x = head.min.x + 6.0;
    if s == Sheet::Menu && !menu_path.is_empty() {
        let b = Rect::from_min_size(pos2(x, head.min.y + 1.0), vec2(TOUCH, TOUCH));
        if icon_button(ui, b, Icon::ChevronLeft, false, true, &t, "sheet.up").clicked()
            && let Some(m) = app.mobile.as_mut()
        {
            m.menu_path.pop();
        }
        x = b.max.x;
    }
    let title = if s == Sheet::Menu && !menu_path.is_empty() { menu_path.join(" › ") } else { crate::i18n::t(sheet_title(s)).to_string() };
    let mut job = egui::text::LayoutJob::simple_singleline(title, Tokens::semibold(15.0), t.text);
    job.wrap.max_width = (head.max.x - x - 110.0).max(20.0);
    job.wrap.max_rows = 1;
    let g = ui.painter().layout_job(job);
    ui.painter().galley(pos2(x + 8.0, head.center().y - g.size().y / 2.0), g, t.text);
    let close = Rect::from_min_size(pos2(head.max.x - TOUCH - 2.0, head.min.y + 1.0), vec2(TOUCH, TOUCH));
    if icon_button(ui, close, Icon::Close, false, true, &t, "sheet.close").clicked()
        && let Some(m) = app.mobile.as_mut()
    {
        m.sheet = None;
        m.sheet_tall = false;
    }
    app.auto.add("mobile.sheet.close", close, "Close");
    if portrait {
        let tall = app.mobile.as_ref().is_some_and(|m| m.sheet_tall);
        let grow = close.translate(vec2(-TOUCH, 0.0));
        if icon_button(ui, grow, if tall { Icon::ChevronDown } else { Icon::ArrowUp }, tall, true, &t, "sheet.grow").clicked()
            && let Some(m) = app.mobile.as_mut()
        {
            m.sheet_tall = !m.sheet_tall;
        }
        app.auto.add("mobile.sheet.grow", grow, "Expand");
    }
    let body = Rect::from_min_max(pos2(r.min.x, head.max.y), r.max);
    match s {
        Sheet::Panel(p) => panel_in(app, ui, p, body),
        Sheet::Media => media_sheet(app, ui, ctx, body),
        Sheet::Panels => panels_sheet(app, ui, body),
        Sheet::Menu => menu_sheet(app, ui, ctx, body),
        Sheet::Audio => actions(app, ui, ctx, body, "audio", &AUDIO_ACTIONS),
        Sheet::Text => actions(app, ui, ctx, body, "text", &TEXT_ACTIONS),
        Sheet::Transitions => actions(app, ui, ctx, body, "transitions", &TRANSITION_ACTIONS),
        Sheet::Canvas => canvas_sheet(app, ui, ctx, body),
    }
}

/// A row in an action list.
#[derive(Clone, Copy)]
enum Row {
    Command(&'static str, Value0),
    Open(Sheet),
    AddAudio,
}

/// Command parameters that can live in a `const` (built when run).
#[derive(Clone, Copy)]
enum Value0 {
    None,
    NewText,
    VerticalText,
}

impl Value0 {
    fn json(self) -> Value {
        match self {
            Value0::None => json!({}),
            Value0::NewText => json!({"text": "Your text"}),
            Value0::VerticalText => json!({"text": "Your text", "vertical": true}),
        }
    }
}

const AUDIO_ACTIONS: [(&str, &str, Icon, Row); 7] = [
    ("Add music or sound", "From the phone's music, downloads or recordings", Icon::Plus, Row::AddAudio),
    ("Record voice-over", "Records from the microphone at the playhead (tap again to stop)", Icon::Mic, Row::Command("voiceover.recordToggle", Value0::None)),
    ("Text to Speech", "Narration from typed text", Icon::Chat, Row::Open(Sheet::Panel(PanelKind::TextToSpeech))),
    ("Essential Sound", "Dialogue, music and ambience presets, loudness", Icon::Sparkle, Row::Open(Sheet::Panel(PanelKind::EssentialSound))),
    ("Track mixer", "Faders, pan, mute, solo, effects", Icon::Speaker, Row::Open(Sheet::Panel(PanelKind::AudioTrackMixer))),
    ("Clip mixer", "The selected clips' levels", Icon::Speaker, Row::Open(Sheet::Panel(PanelKind::AudioClipMixer))),
    ("Meters", "Peak and loudness (LUFS) meters", Icon::Audio, Row::Open(Sheet::Panel(PanelKind::AudioMeters))),
];

const TEXT_ACTIONS: [(&str, &str, Icon, Row); 7] = [
    ("Add text", "A title at the playhead; edit it in Properties", Icon::Type, Row::Command("graphics.newText", Value0::NewText)),
    ("Vertical text", "Top-to-bottom text", Icon::Type, Row::Command("graphics.newText", Value0::VerticalText)),
    ("Rectangle", "A shape layer", Icon::Rectangle, Row::Command("graphics.newRectangle", Value0::None)),
    ("Ellipse", "A shape layer", Icon::Ellipse, Row::Command("graphics.newEllipse", Value0::None)),
    ("Properties", "Font, size, colour, stroke and shadow of the selected text", Icon::Adjust, Row::Open(Sheet::Panel(PanelKind::Properties))),
    ("Templates", "Essential Graphics titles and lower thirds", Icon::Grid, Row::Open(Sheet::Panel(PanelKind::EssentialGraphics))),
    ("Captions", "Subtitles: type, import SRT/VTT, or transcribe", Icon::Captions, Row::Open(Sheet::Panel(PanelKind::Text))),
];

const TRANSITION_ACTIONS: [(&str, &str, Icon, Row); 5] = [
    ("Cross dissolve", "At the cut nearest the playhead", Icon::Sparkle, Row::Command("sequence.applyVideoTransition", Value0::None)),
    ("Audio crossfade", "At the cut nearest the playhead", Icon::Audio, Row::Command("sequence.applyAudioTransition", Value0::None)),
    ("To every selected cut", "The default transitions on the selected clips' edits", Icon::Sparkle, Row::Command("trim.applyDefaultTransition", Value0::None)),
    ("All transitions", "Wipes, pushes, slides, zooms, irises and more (double-tap one to apply)", Icon::Grid, Row::Open(Sheet::Panel(PanelKind::Effects))),
    ("Remove transition", "From the cut under the playhead", Icon::Trash, Row::Command("sequence.removeTransition", Value0::None)),
];

/// A list of big rows: title, a line of explanation, and what a tap does.
fn actions(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, body: Rect, id: &str, rows: &[(&'static str, &'static str, Icon, Row)]) {
    let t = app.tokens;
    let mut fired = None;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt(("mobile-actions", id)));
    child.set_clip_rect(body);
    egui::ScrollArea::vertical().id_salt(("mobile-actions-scroll", id)).auto_shrink([false, false]).show(&mut child, |ui| {
        ui.add_space(4.0);
        for (i, (title, sub, icon, row)) in rows.iter().enumerate() {
            let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 62.0), Sense::click());
            if resp.is_pointer_button_down_on() {
                ui.painter().rect_filled(r.shrink2(vec2(8.0, 2.0)), 10.0, t.hover);
            }
            let tile = Rect::from_min_size(pos2(r.min.x + 14.0, r.center().y - 20.0), vec2(40.0, 40.0));
            ui.painter().rect_filled(tile, 10.0, t.accent.gamma_multiply(0.2));
            icons::paint(ui.painter(), tile.shrink(10.0), *icon, t.focus);
            ui.painter().text(pos2(tile.max.x + 12.0, r.center().y - 9.0), Align2::LEFT_CENTER, crate::i18n::t(title), Tokens::semibold(15.0), t.text);
            let mut job = egui::text::LayoutJob::simple_singleline(crate::i18n::t(sub).to_string(), Tokens::ui(12.0), t.text_dim);
            job.wrap.max_width = r.max.x - tile.max.x - 24.0;
            job.wrap.max_rows = 1;
            let g = ui.painter().layout_job(job);
            ui.painter().galley(pos2(tile.max.x + 12.0, r.center().y + 2.0), g, t.text_dim);
            if resp.clicked() {
                fired = Some(*row);
            }
            let _ = i;
        }
    });
    match fired {
        Some(Row::Command(cmd, p)) => {
            run(app, ctx, cmd, p.json());
            // a new title is edited in Properties
            if cmd == "graphics.newText"
                && let Some(m) = app.mobile.as_mut()
            {
                m.sheet = Some(Sheet::Panel(PanelKind::Properties));
            }
        }
        Some(Row::Open(s)) => {
            if let Some(m) = app.mobile.as_mut() {
                m.sheet = Some(s);
            }
        }
        Some(Row::AddAudio) => add_media(app, MediaKind::Audio),
        None => {}
    }
}

/// Frame size presets for social and screen formats, and frame rates.
const CANVAS: [(&str, &str, u32, u32); 7] = [
    ("9:16", "Reels, Shorts, TikTok, Stories", 1080, 1920),
    ("16:9", "YouTube, TV", 1920, 1080),
    ("1:1", "Square posts", 1080, 1080),
    ("4:5", "Portrait posts", 1080, 1350),
    ("4:3", "Classic", 1440, 1080),
    ("21:9", "Cinema", 2560, 1080),
    ("4K 16:9", "Ultra HD", 3840, 2160),
];
const RATES: [f64; 6] = [23.976, 24.0, 25.0, 30.0, 50.0, 60.0];

fn canvas_sheet(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, body: Rect) {
    let t = app.tokens;
    let Some((w, h, fps)) = app.session.active_sequence().map(|q| (q.settings.width, q.settings.height, q.settings.frame_rate.as_f64())) else {
        ui.painter().text(body.center_top() + vec2(0.0, 40.0), Align2::CENTER_CENTER, crate::i18n::t("Add some media first"), Tokens::ui(14.0), t.text_dim);
        return;
    };
    let mut set: Option<Value> = None;
    let mut settings = false;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt("mobile-canvas"));
    child.set_clip_rect(body);
    egui::ScrollArea::vertical().id_salt("mobile-canvas-scroll").auto_shrink([false, false]).show(&mut child, |ui| {
        ui.add_space(8.0);
        let cols = ((ui.available_width() - 16.0) / 104.0).floor().max(2.0) as usize;
        let cw = (ui.available_width() - 16.0) / cols as f32;
        for chunk in CANVAS.chunks(cols) {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.x = 0.0;
                for (name, sub, cw_px, ch_px) in chunk {
                    let (c, resp) = ui.allocate_exact_size(vec2(cw, 112.0), Sense::click());
                    let on = (w, h) == (*cw_px, *ch_px);
                    let card = c.shrink(5.0);
                    ui.painter().rect_filled(card, 12.0, if on { t.accent.gamma_multiply(0.3) } else { t.tl_header_bg });
                    // the shape, to scale
                    let k = 46.0 / (*cw_px).max(*ch_px) as f32;
                    let shape = Rect::from_center_size(pos2(card.center().x, card.min.y + 34.0), vec2(*cw_px as f32 * k, *ch_px as f32 * k));
                    ui.painter().rect_stroke(shape, 3.0, Stroke::new(2.0, if on { t.focus } else { t.text_dim }), StrokeKind::Middle);
                    ui.painter().text(pos2(card.center().x, card.max.y - 32.0), Align2::CENTER_CENTER, *name, Tokens::semibold(14.0), t.text);
                    let mut job = egui::text::LayoutJob::simple_singleline(crate::i18n::t(sub).to_string(), Tokens::ui(10.5), t.text_dim);
                    job.wrap.max_width = card.width() - 8.0;
                    job.wrap.max_rows = 1;
                    job.halign = egui::Align::Center;
                    let g = ui.painter().layout_job(job);
                    ui.painter().galley(pos2(card.center().x, card.max.y - 18.0), g, t.text_dim);
                    if resp.clicked() {
                        set = Some(json!({"width": cw_px, "height": ch_px}));
                    }
                }
            });
        }
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            ui.add_space(12.0);
            ui.label(egui::RichText::new(crate::i18n::t("Frame rate")).color(t.text_dim));
            for r in RATES {
                let on = (fps - r).abs() < 0.01;
                let label = if r.fract() == 0.0 { format!("{r:.0}") } else { format!("{r:.3}") };
                if ui.add(egui::Button::selectable(on, label).min_size(vec2(52.0, 36.0))).clicked() {
                    set = Some(json!({"fps": r}));
                }
            }
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(egui::RichText::new(format!("{w} × {h}")).color(t.text));
            if ui.add(egui::Button::new(crate::i18n::t("Sequence Settings…")).min_size(vec2(0.0, 36.0))).clicked() {
                settings = true;
            }
        });
    });
    if let Some(p) = set
        && let Err(e) = app.session.execute("sequence.settings", p)
    {
        app.toast = Some((e.to_string(), ctx.input(|i| i.time)));
    }
    if settings {
        run(app, ctx, "sequence.settings", json!({}));
    }
}

fn media_sheet(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, body: Rect) {
    let t = app.tokens;
    let row = Rect::from_min_size(body.min, vec2(body.width(), 56.0));
    let add = Rect::from_min_size(row.min + vec2(10.0, 8.0), vec2(((row.width() - 30.0) / 2.0).min(200.0), 40.0));
    if text_button(ui, add, crate::i18n::t("Add from phone"), true, &t, "media.add").clicked() {
        add_media(app, MediaKind::Any);
    }
    app.auto.add("mobile.media.add", add, "Add from phone");
    let selected: Vec<u64> = app.session.state.project_selection.iter().map(|i| i.0).collect();
    let place = add.translate(vec2(add.width() + 10.0, 0.0));
    let label = if selected.is_empty() { crate::i18n::t("Add to timeline").to_string() } else { format!("{} ({})", crate::i18n::t("Add to timeline"), selected.len()) };
    let resp = text_button(ui, place, &label, false, &t, "media.place");
    ui.painter().rect_stroke(place, place.height() / 2.0, Stroke::new(1.0, if selected.is_empty() { t.text_faint } else { t.text_dim }), StrokeKind::Inside);
    if resp.clicked() {
        if selected.is_empty() {
            app.toast = Some((crate::i18n::t("Tap a clip below first").to_string(), ctx.input(|i| i.time)));
        } else if let Err(e) = place_on_timeline(app, &selected) {
            app.toast = Some((e, ctx.input(|i| i.time)));
        } else if let Some(m) = app.mobile.as_mut() {
            m.sheet = None;
        }
    }
    app.auto.add("mobile.media.place", place, "Add to timeline");
    let rest = Rect::from_min_max(pos2(body.min.x, row.max.y), body.max);
    panel_in(app, ui, PanelKind::Project, rest);
}

fn panels_sheet(app: &mut FilmcraftApp, ui: &mut egui::Ui, body: Rect) {
    let t = app.tokens;
    let panels: Vec<PanelKind> = PanelKind::ALL.iter().copied().filter(|p| !matches!(p, PanelKind::Program | PanelKind::Timeline | PanelKind::Tools | PanelKind::Libraries)).collect();
    let mut open = None;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt("mobile-panels"));
    child.set_clip_rect(body);
    egui::ScrollArea::vertical().id_salt("mobile-panels-scroll").auto_shrink([false, false]).show(&mut child, |ui| {
        ui.add_space(6.0);
        let cols = ((ui.available_width() - 12.0) / 150.0).floor().max(2.0) as usize;
        let cw = (ui.available_width() - 12.0) / cols as f32;
        for chunk in panels.chunks(cols) {
            ui.horizontal(|ui| {
                ui.add_space(6.0);
                ui.spacing_mut().item_spacing.x = 0.0;
                for p in chunk {
                    let (c, resp) = ui.allocate_exact_size(vec2(cw, 54.0), Sense::click());
                    let card = c.shrink(4.0);
                    ui.painter().rect_filled(card, 10.0, if resp.is_pointer_button_down_on() { t.pressed } else { t.tl_header_bg });
                    let mut job = egui::text::LayoutJob::simple_singleline(crate::i18n::t(p.title()).to_string(), Tokens::ui(13.5), t.text);
                    job.wrap.max_width = card.width() - 16.0;
                    job.wrap.max_rows = 1;
                    let g = ui.painter().layout_job(job);
                    ui.painter().galley(pos2(card.min.x + 12.0, card.center().y - g.size().y / 2.0), g, t.text);
                    if resp.clicked() {
                        open = Some(*p);
                    }
                }
            });
        }
    });
    if let Some(p) = open {
        app.ui.focused = p;
        if let Some(m) = app.mobile.as_mut() {
            m.sheet = Some(Sheet::Panel(p));
        }
    }
}

/// The menu bar as a drill-down list: menus, their submenus, then commands.
fn menu_sheet(app: &mut FilmcraftApp, ui: &mut egui::Ui, ctx: &egui::Context, body: Rect) {
    let t = app.tokens;
    let path = app.mobile.as_ref().map(|m| m.menu_path.clone()).unwrap_or_default();
    let items = crate::menus::menu_items(app);
    // rows: submenus at this level (in first-seen order), then the commands here
    let mut subs: Vec<String> = Vec::new();
    let mut here: Vec<crate::menus::MenuItem> = Vec::new();
    if path.is_empty() {
        for m in crate::menus::MENUS {
            if items.iter().any(|it| it.path.first().map(String::as_str) == Some(m)) {
                subs.push(m.to_string());
            }
        }
    } else {
        for it in &items {
            if it.path.len() > path.len() && it.path[..path.len()] == path[..] {
                let s = it.path[path.len()].clone();
                if !subs.contains(&s) {
                    subs.push(s);
                }
            } else if it.path == path {
                here.push(it.clone());
            }
        }
    }
    let mut go: Option<String> = None;
    let mut invoke: Option<String> = None;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).id_salt("mobile-menu"));
    child.set_clip_rect(body);
    egui::ScrollArea::vertical().id_salt(("mobile-menu-scroll", path.join("/"))).auto_shrink([false, false]).show(&mut child, |ui| {
        let w = ui.available_width();
        for s in &subs {
            let (r, resp) = ui.allocate_exact_size(vec2(w, 50.0), Sense::click());
            if resp.is_pointer_button_down_on() {
                ui.painter().rect_filled(r, 0.0, t.hover);
            }
            ui.painter().text(pos2(r.min.x + 18.0, r.center().y), Align2::LEFT_CENTER, crate::i18n::t(s), Tokens::ui(15.0), t.text);
            icons::paint(ui.painter(), Rect::from_center_size(pos2(r.max.x - 22.0, r.center().y), vec2(14.0, 14.0)), Icon::ChevronRight, t.text_dim);
            ui.painter().line_segment([pos2(r.min.x + 18.0, r.max.y), r.right_bottom()], Stroke::new(1.0, t.separator));
            if resp.clicked() {
                go = Some(s.clone());
            }
        }
        for it in &here {
            let (r, resp) = ui.allocate_exact_size(vec2(w, 48.0), if it.enabled { Sense::click() } else { Sense::hover() });
            if resp.is_pointer_button_down_on() {
                ui.painter().rect_filled(r, 0.0, t.hover);
            }
            let c = if it.enabled { t.text } else { t.text_faint };
            if it.checked == Some(true) {
                ui.painter().text(pos2(r.min.x + 16.0, r.center().y), Align2::LEFT_CENTER, "✓", Tokens::semibold(14.0), t.focus);
            }
            let mut job = egui::text::LayoutJob::simple_singleline(it.label.clone(), Tokens::ui(15.0), c);
            job.wrap.max_width = w - 60.0;
            job.wrap.max_rows = 1;
            let g = ui.painter().layout_job(job);
            ui.painter().galley(pos2(r.min.x + 38.0, r.center().y - g.size().y / 2.0), g, c);
            ui.painter().line_segment([pos2(r.min.x + 38.0, r.max.y), r.right_bottom()], Stroke::new(1.0, t.separator));
            if resp.clicked() && it.enabled {
                invoke = Some(it.id.clone());
            }
        }
    });
    if let Some(s) = go
        && let Some(m) = app.mobile.as_mut()
    {
        m.menu_path.push(s);
    }
    if let Some(id) = invoke {
        if let Some(m) = app.mobile.as_mut() {
            m.sheet = None;
            m.menu_path.clear();
        }
        run(app, ctx, &id, json!({}));
    }
}

/// The preview alone, filling the screen; a tap shows or hides its controls.
fn fullscreen(app: &mut FilmcraftApp, ui: &mut egui::Ui, full: Rect) {
    let ctx = ui.ctx().clone();
    let t = app.tokens;
    ui.painter().rect_filled(full, 0.0, Color32::BLACK);
    program(app, ui, full);
    let id = egui::Id::new("mobile-fullscreen-controls");
    let shown = ctx.data(|d| d.get_temp::<bool>(id).unwrap_or(true));
    let tap = ui.interact(full, egui::Id::new("mobile-fullscreen-tap"), Sense::click());
    if tap.clicked() {
        ctx.data_mut(|d| d.insert_temp(id, !shown));
    }
    if !shown {
        return;
    }
    let insets = app.mobile.as_ref().map(|m| m.insets).unwrap_or_default();
    let exit = Rect::from_min_size(pos2(full.max.x - insets.right - 56.0, full.min.y + insets.top + 8.0), vec2(48.0, 48.0));
    ui.painter().circle_filled(exit.center(), 22.0, Color32::from_black_alpha(140));
    if icon_button(ui, exit, Icon::Close, false, true, &t, "fullscreen.exit").clicked()
        && let Some(m) = app.mobile.as_mut()
    {
        m.fullscreen = false;
    }
    let play = Rect::from_center_size(pos2(full.center().x, full.max.y - insets.bottom - 48.0), vec2(64.0, 64.0));
    ui.painter().circle_filled(play.center(), 30.0, Color32::from_black_alpha(150));
    let resp = ui.interact(play, egui::Id::new("mobile-fullscreen-play"), Sense::click());
    icons::paint(ui.painter(), Rect::from_center_size(play.center(), vec2(24.0, 24.0)), if app.playback.playing { Icon::Pause } else { Icon::Play }, Color32::WHITE);
    if resp.clicked() {
        run(app, &ctx, "playback.toggle", json!({}));
    }
}

/// Toasts, status messages and the Quick Export popup.
fn overlays(app: &mut FilmcraftApp, _ui: &mut egui::Ui, ctx: &egui::Context, area: Rect) {
    // Quick Export, under the Export button
    let anchor = pos2((area.max.x - 352.0).max(area.min.x + 4.0), area.min.y + 56.0);
    crate::panels::export_mode::quick_export(app, ctx, anchor);
    // a message: the newest toast, else the status line
    let now = ctx.input(|i| i.time);
    let msg = match &app.toast {
        Some((m, at)) if now - at < 3.5 => Some(m.clone()),
        _ if !app.ui.status.is_empty() => Some(app.ui.status.clone()),
        _ => None,
    };
    if let Some(m) = &app.toast
        && now - m.1 >= 3.5
    {
        app.toast = None;
    }
    let Some(msg) = msg else { return };
    let key = egui::Id::new("mobile-status-seen");
    let (seen, since) = ctx.data(|d| d.get_temp::<(String, f64)>(key)).unwrap_or_default();
    let since = if seen == msg { since } else { now };
    ctx.data_mut(|d| d.insert_temp(key, (msg.clone(), since)));
    if now - since > 4.0 {
        if app.ui.status == msg {
            app.ui.status.clear();
        }
        return;
    }
    ctx.request_repaint_after(std::time::Duration::from_millis(250));
    egui::Area::new(egui::Id::new("mobile-toast")).order(egui::Order::Tooltip).interactable(false).fixed_pos(area.min).show(ctx, |ui| {
        let mut job = egui::text::LayoutJob::simple(msg, Tokens::ui(13.0), Color32::WHITE, (area.width() - 64.0).max(80.0));
        job.halign = egui::Align::Center;
        let g = ui.painter().layout_job(job);
        let size = g.size() + vec2(28.0, 18.0);
        let r = Rect::from_center_size(pos2(area.center().x, area.max.y - 110.0), size);
        ui.painter().rect_filled(r, 14.0, Color32::from_rgba_unmultiplied(30, 26, 30, 235));
        ui.painter().galley(pos2(r.center().x, r.min.y + 9.0), g, Color32::WHITE);
    });
}

/// Finished exports: tell the user where the file went, and hand it to the host (the Android
/// media scanner, so it shows in the gallery).
fn announce_exports(app: &mut FilmcraftApp, ctx: &egui::Context) {
    let done: Vec<(u64, String)> = app
        .session
        .jobs
        .iter()
        .filter(|j| app.mobile.as_ref().is_some_and(|m| !m.announced.contains(&j.id)))
        .filter_map(|j| {
            let res = j.result.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match res.as_ref() {
                Some(Ok(r)) => Some((j.id, r.path.clone())),
                Some(Err(_)) => Some((j.id, String::new())),
                None => None,
            }
        })
        .collect();
    for (id, path) in done {
        if let Some(m) = app.mobile.as_mut() {
            m.announced.push(id);
        }
        if path.is_empty() || !std::path::Path::new(&path).is_file() {
            continue;
        }
        if let Some(f) = app.hooks.file_written.as_mut() {
            f(&path);
        }
        app.toast = Some((crate::i18n::fmt(crate::i18n::t("Saved to {path}"), &[("path", path.as_str())]), ctx.input(|i| i.time)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbars_fit_their_items() {
        // every toolbar item has a label and an icon, and the selection toolbar starts with Done
        assert!(matches!(CLIP_TOOLS[0].2, Act::Deselect));
        assert!(MAIN_TOOLS.iter().chain(CLIP_TOOLS.iter()).all(|(l, ..)| !l.is_empty()));
    }

    #[test]
    fn canvas_presets_are_even_sizes() {
        // H.264 4:2:0 needs even frame sizes
        assert!(CANVAS.iter().all(|(_, _, w, h)| w % 2 == 0 && h % 2 == 0));
    }
}
