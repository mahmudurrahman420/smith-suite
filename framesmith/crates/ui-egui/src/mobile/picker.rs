//! The in-app file picker of the mobile layout.
//!
//! Phones have no file dialog the app can block on, so the mobile layout answers every
//! [`FileDialog`] itself (it is the host's [`HostHooks::run_dialog`](crate::HostHooks::run_dialog)):
//! a full-screen, touch-sized browser of the phone's storage with shortcuts to the camera roll,
//! videos, pictures, downloads and music. The frame loop keeps running while it is open and the
//! answer goes back on the dialog's channel, exactly like a native dialog on the desktop.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, PoisonError};

use egui::{Align2, Color32, Rect, Sense, Stroke, pos2, vec2};

use crate::FilmcraftApp;
use crate::icons::{self, Icon};
use crate::native_dialogs::FileDialog;
use crate::theme::Tokens;

/// Dialogs asked for since the last frame, with where to send the answer.
pub type Queue = Arc<Mutex<Vec<(FileDialog, Sender<Vec<String>>)>>>;

/// The `run_dialog` hook that hands dialogs to the picker.
pub fn hook(queue: Queue) -> Box<dyn FnMut(FileDialog) -> Receiver<Vec<String>>> {
    Box::new(move |dialog| {
        let (tx, rx) = channel();
        queue.lock().unwrap_or_else(PoisonError::into_inner).push((dialog, tx));
        rx
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Folder,
    Video,
    Audio,
    Image,
    Project,
    Other,
}

impl Kind {
    fn of(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
        match ext.as_str() {
            "mp4" | "mov" | "m4v" | "mkv" | "webm" | "avi" | "3gp" | "mts" | "m2ts" | "mxf" | "ts" | "mpg" | "mpeg" => Kind::Video,
            "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg" | "opus" | "aif" | "aiff" | "caf" => Kind::Audio,
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "heic" | "tif" | "tiff" | "bmp" | "dng" => Kind::Image,
            "fcproj" => Kind::Project,
            _ => Kind::Other,
        }
    }
    fn icon(self) -> Icon {
        match self {
            Kind::Folder => Icon::Folder,
            Kind::Video => Icon::Film,
            Kind::Audio => Icon::Audio,
            Kind::Image => Icon::Image,
            Kind::Project => Icon::Sequence,
            Kind::Other => Icon::NewItem,
        }
    }
}

#[derive(Clone, Debug)]
struct Entry {
    path: PathBuf,
    name: String,
    kind: Kind,
    size: u64,
    modified: Option<std::time::SystemTime>,
}

/// The dialog on screen.
struct Active {
    dialog: FileDialog,
    answer: Sender<Vec<String>>,
    dir: PathBuf,
    chosen: Vec<PathBuf>,
    /// The file name typed for a save.
    name: String,
    entries: Vec<Entry>,
    error: Option<String>,
}

/// Picker state: the dialogs waiting and the one on screen.
pub struct Picker {
    pub queue: Queue,
    active: Option<Active>,
    /// The folder the last import came from: the next one starts there.
    last_dir: Option<PathBuf>,
}

impl Default for Picker {
    fn default() -> Self {
        Self { queue: Arc::new(Mutex::new(Vec::new())), active: None, last_dir: None }
    }
}

/// The user's storage root: `$HOME` (the shared storage on Android, set by the host).
fn home() -> PathBuf {
    std::env::var_os("HOME").map(PathBuf::from).filter(|p| p.is_dir()).unwrap_or_else(|| PathBuf::from("/"))
}

/// Shortcuts to the usual media folders that exist on this device.
fn places() -> Vec<(&'static str, Icon, PathBuf)> {
    let h = home();
    let camera = if h.join("DCIM/Camera").is_dir() { h.join("DCIM/Camera") } else { h.join("DCIM") };
    let all = [
        ("Camera", Icon::Camera, camera),
        ("Videos", Icon::Film, h.join("Movies")),
        ("Pictures", Icon::Image, h.join("Pictures")),
        ("Downloads", Icon::Import, h.join("Download")),
        ("Music", Icon::Audio, h.join("Music")),
        ("Documents", Icon::Folder, h.join("Documents")),
        ("Phone", Icon::Drive, h.clone()),
    ];
    all.into_iter().filter(|(_, _, p)| p.is_dir()).collect()
}

fn allowed(dialog: &FileDialog) -> Vec<String> {
    match dialog {
        FileDialog::OpenFiles { exts } | FileDialog::OpenFile { exts, .. } | FileDialog::SaveAs { exts, .. } | FileDialog::Relink { exts, .. } => {
            exts.iter().map(|e| e.trim_start_matches('.').to_ascii_lowercase()).filter(|e| !e.is_empty() && e != "*").collect()
        }
        FileDialog::OpenProject | FileDialog::SaveProject { .. } => vec!["fcproj".into()],
        FileDialog::Folder { .. } => Vec::new(),
    }
}

fn wants_folder(dialog: &FileDialog) -> bool {
    matches!(dialog, FileDialog::Folder { .. })
}

fn is_save(dialog: &FileDialog) -> bool {
    matches!(dialog, FileDialog::SaveAs { .. } | FileDialog::SaveProject { .. })
}

fn multiple(dialog: &FileDialog) -> bool {
    matches!(dialog, FileDialog::OpenFiles { .. })
}

fn title(dialog: &FileDialog) -> &'static str {
    match dialog {
        FileDialog::OpenFiles { .. } => "Add Media",
        FileDialog::OpenFile { .. } | FileDialog::Relink { .. } => "Choose a File",
        FileDialog::OpenProject => "Open Project",
        FileDialog::SaveProject { .. } => "Save Project",
        FileDialog::SaveAs { .. } => "Save As",
        FileDialog::Folder { .. } => "Choose a Folder",
    }
}

fn list(dir: &Path, exts: &[String], folders_only: bool) -> Result<Vec<Entry>, String> {
    let rd = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut out = Vec::new();
    for e in rd.flatten().take(5000) {
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let path = e.path();
        let meta = e.metadata().ok();
        let is_dir = meta.as_ref().is_some_and(std::fs::Metadata::is_dir);
        let kind = if is_dir { Kind::Folder } else { Kind::of(&path) };
        if !is_dir {
            if folders_only {
                continue;
            }
            let ext = path.extension().and_then(|x| x.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
            if !exts.is_empty() && !exts.contains(&ext) {
                continue;
            }
        }
        out.push(Entry {
            path,
            name,
            kind,
            size: meta.as_ref().map_or(0, std::fs::Metadata::len),
            modified: meta.and_then(|m| m.modified().ok()),
        });
    }
    // folders A–Z, then files newest first (the camera roll reads like a gallery)
    out.sort_by(|a, b| match (a.kind == Kind::Folder, b.kind == Kind::Folder) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (true, true) => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        (false, false) => b.modified.cmp(&a.modified).then_with(|| a.name.cmp(&b.name)),
    });
    Ok(out)
}

fn human_size(n: u64) -> String {
    let n = n as f64;
    if n >= 1e9 {
        format!("{:.1} GB", n / 1e9)
    } else if n >= 1e6 {
        format!("{:.1} MB", n / 1e6)
    } else if n >= 1e3 {
        format!("{:.0} KB", n / 1e3)
    } else {
        format!("{n:.0} B")
    }
}

impl Picker {
    /// Whether a picker is on screen.
    pub fn open(&self) -> bool {
        self.active.is_some()
    }

    /// Take the next asked-for dialog, if none is showing.
    fn take_next(&mut self) {
        if self.active.is_some() {
            return;
        }
        let next = {
            let mut q = self.queue.lock().unwrap_or_else(PoisonError::into_inner);
            if q.is_empty() { None } else { Some(q.remove(0)) }
        };
        let Some((dialog, answer)) = next else { return };
        let (dir, name) = match &dialog {
            FileDialog::SaveAs { name, .. } | FileDialog::SaveProject { name } => {
                let p = Path::new(name);
                let dir = p.parent().filter(|d| d.is_dir()).map(Path::to_path_buf).unwrap_or_else(|| {
                    let movies = home().join(if matches!(dialog, FileDialog::SaveProject { .. }) { "Documents" } else { "Movies" });
                    if movies.is_dir() { movies } else { home() }
                });
                (dir, p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| name.clone()))
            }
            FileDialog::Folder { at: Some(at) } if Path::new(at).is_dir() => (PathBuf::from(at), String::new()),
            FileDialog::OpenProject => {
                let d = home().join("Documents/Framesmith");
                (if d.is_dir() { d } else { home() }, String::new())
            }
            _ => (self.last_dir.clone().filter(|d| d.is_dir()).or_else(|| places().first().map(|p| p.2.clone())).unwrap_or_else(home), String::new()),
        };
        let mut a = Active { dialog, answer, dir, chosen: Vec::new(), name, entries: Vec::new(), error: None };
        refresh(&mut a);
        self.active = Some(a);
    }

    /// Close the picker with `paths` as its answer (none: cancelled).
    fn finish(&mut self, paths: Vec<PathBuf>) {
        if let Some(a) = self.active.take() {
            if !paths.is_empty() && !is_save(&a.dialog) && !wants_folder(&a.dialog) {
                self.last_dir = Some(a.dir.clone());
            }
            let _ = a.answer.send(paths.into_iter().map(|p| p.to_string_lossy().to_string()).collect());
        }
    }

    /// Cancel the picker (the back button).
    pub fn cancel(&mut self) {
        self.finish(Vec::new());
    }
}

fn refresh(a: &mut Active) {
    let exts = allowed(&a.dialog);
    match list(&a.dir, &exts, wants_folder(&a.dialog)) {
        Ok(e) => {
            a.entries = e;
            a.error = None;
        }
        Err(e) => {
            a.entries.clear();
            a.error = Some(e);
        }
    }
}

/// Draw the picker over `screen` when one is open. Returns true when it covered the screen.
pub fn show(app: &mut FilmcraftApp, ctx: &egui::Context, screen: Rect) -> bool {
    let Some(m) = app.mobile.as_mut() else { return false };
    let was_open = m.picker.open();
    m.picker.take_next();
    if !m.picker.open() {
        return false;
    }
    if !was_open && let Some(f) = app.hooks.media_access.as_mut() {
        f();
    }
    let t = app.tokens;
    let Some(insets) = app.mobile.as_ref().map(|m| m.insets) else { return false };
    let mut finish: Option<Vec<PathBuf>> = None;
    let mut go: Option<PathBuf> = None;
    egui::Area::new(egui::Id::new("mobile-picker")).order(egui::Order::Foreground).fixed_pos(screen.min).show(ctx, |ui| {
        let Some(a) = app.mobile.as_mut().and_then(|m| m.picker.active.as_mut()) else { return };
        let (rect, _) = ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
        let p = ui.painter().clone();
        p.rect_filled(rect, 0.0, t.panel_bg);
        let inner = Rect::from_min_max(rect.min + vec2(insets.left, insets.top), rect.max - vec2(insets.right, insets.bottom));
        // ---- title bar: Cancel | title | action
        let bar = Rect::from_min_size(inner.min, vec2(inner.width(), 52.0));
        p.rect_filled(bar, 0.0, t.header_bg);
        let cancel = Rect::from_min_size(bar.min + vec2(4.0, 4.0), vec2(88.0, 44.0));
        if crate::mobile::text_button(ui, cancel, crate::i18n::t("Cancel"), false, &t, "picker.cancel").clicked() {
            finish = Some(Vec::new());
        }
        p.text(bar.center(), Align2::CENTER_CENTER, crate::i18n::t(title(&a.dialog)), Tokens::semibold(16.0), t.text);
        let action = if wants_folder(&a.dialog) {
            Some(crate::i18n::t("Choose").to_string())
        } else if is_save(&a.dialog) {
            Some(crate::i18n::t("Save").to_string())
        } else if multiple(&a.dialog) && !a.chosen.is_empty() {
            Some(format!("{} ({})", crate::i18n::t("Add"), a.chosen.len()))
        } else {
            None
        };
        if let Some(label) = action {
            let r = Rect::from_min_size(pos2(bar.max.x - 112.0, bar.min.y + 6.0), vec2(106.0, 40.0));
            if crate::mobile::text_button(ui, r, &label, true, &t, "picker.confirm").clicked() {
                if wants_folder(&a.dialog) {
                    finish = Some(vec![a.dir.clone()]);
                } else if is_save(&a.dialog) {
                    let mut name = a.name.trim().to_string();
                    if !name.is_empty() {
                        let exts = allowed(&a.dialog);
                        let has = Path::new(&name).extension().and_then(|e| e.to_str()).is_some_and(|e| exts.is_empty() || exts.contains(&e.to_ascii_lowercase()));
                        if !has && let Some(e) = exts.first() {
                            name = format!("{name}.{e}");
                        }
                        finish = Some(vec![a.dir.join(name)]);
                    }
                } else {
                    finish = Some(a.chosen.clone());
                }
            }
        }
        let mut y = bar.max.y + 6.0;
        // ---- places (a scrolling row of chips)
        let chips = Rect::from_min_size(pos2(inner.min.x, y), vec2(inner.width(), 44.0));
        ui.scope_builder(egui::UiBuilder::new().max_rect(chips), |ui| {
            egui::ScrollArea::horizontal().id_salt("picker-places").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    for (label, icon, path) in places() {
                        let label = crate::i18n::t(label);
                        let w = ui.painter().layout_no_wrap(label.to_string(), Tokens::ui(14.0), t.text).size().x + 44.0;
                        let (r, resp) = ui.allocate_exact_size(vec2(w, 38.0), Sense::click());
                        let on = a.dir.starts_with(&path) && (path != home() || a.dir == path);
                        ui.painter().rect_filled(r, 19.0, if on { t.accent } else { t.tl_header_bg });
                        let fg = if on { Color32::WHITE } else { t.text };
                        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 20.0, r.center().y), vec2(16.0, 16.0)), icon, fg);
                        ui.painter().text(pos2(r.min.x + 34.0, r.center().y), Align2::LEFT_CENTER, label, Tokens::ui(14.0), fg);
                        if resp.clicked() {
                            go = Some(path);
                        }
                    }
                    ui.add_space(8.0);
                });
            });
        });
        y = chips.max.y + 4.0;
        // ---- where we are, with Up
        let path_row = Rect::from_min_size(pos2(inner.min.x, y), vec2(inner.width(), 40.0));
        let up = Rect::from_min_size(path_row.min + vec2(6.0, 0.0), vec2(40.0, 40.0));
        let up_resp = ui.interact(up, egui::Id::new("picker-up"), Sense::click());
        let can_up = a.dir.parent().is_some() && a.dir != home();
        icons::paint(&p, up.shrink(10.0), Icon::ArrowUp, if can_up { t.text } else { t.text_faint });
        if up_resp.clicked() && can_up && let Some(parent) = a.dir.parent() {
            go = Some(parent.to_path_buf());
        }
        let shown = a.dir.strip_prefix(home()).map(|r| format!("/{}", r.display())).unwrap_or_else(|_| a.dir.display().to_string());
        let shown = if shown == "/" { crate::i18n::t("Phone").to_string() } else { shown };
        let mut job = egui::text::LayoutJob::simple_singleline(shown, Tokens::ui(13.0), t.text_dim);
        job.wrap.max_width = path_row.width() - 60.0;
        job.wrap.max_rows = 1;
        p.galley(pos2(up.max.x + 6.0, path_row.center().y - 8.0), p.layout_job(job), t.text_dim);
        y = path_row.max.y;
        // ---- file name for a save
        if is_save(&a.dialog) {
            let row = Rect::from_min_size(pos2(inner.min.x + 12.0, y + 4.0), vec2(inner.width() - 24.0, 40.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(row), |ui| {
                ui.add(egui::TextEdit::singleline(&mut a.name).desired_width(row.width()).font(egui::TextStyle::Body).margin(vec2(10.0, 10.0)).hint_text(crate::i18n::t("File name")));
            });
            y = row.max.y + 6.0;
        }
        p.line_segment([pos2(inner.min.x, y), pos2(inner.max.x, y)], Stroke::new(1.0, t.separator));
        // ---- the folder's contents
        let list_rect = Rect::from_min_max(pos2(inner.min.x, y + 1.0), inner.max);
        if let Some(e) = &a.error {
            p.text(list_rect.center_top() + vec2(0.0, 40.0), Align2::CENTER_CENTER, crate::i18n::t("Can't open this folder"), Tokens::ui(14.0), t.text);
            let mut job = egui::text::LayoutJob::simple(e.clone(), Tokens::ui(12.0), t.text_dim, list_rect.width() - 40.0);
            job.halign = egui::Align::Center;
            p.galley(list_rect.center_top() + vec2(0.0, 60.0), p.layout_job(job), t.text_dim);
            return;
        }
        if a.entries.is_empty() {
            p.text(list_rect.center_top() + vec2(0.0, 48.0), Align2::CENTER_CENTER, crate::i18n::t("Nothing here"), Tokens::ui(14.0), t.text_dim);
        }
        let entries = a.entries.clone();
        let row_h = 58.0;
        ui.scope_builder(egui::UiBuilder::new().max_rect(list_rect), |ui| {
            egui::ScrollArea::vertical().id_salt(("picker-list", a.dir.clone())).auto_shrink([false, false]).show_rows(ui, row_h, entries.len(), |ui, range| {
                for e in &entries[range] {
                    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), row_h), Sense::click());
                    let chosen = a.chosen.contains(&e.path);
                    if chosen {
                        ui.painter().rect_filled(r.shrink2(vec2(6.0, 2.0)), 8.0, t.accent.gamma_multiply(0.25));
                    } else if resp.is_pointer_button_down_on() {
                        ui.painter().rect_filled(r.shrink2(vec2(6.0, 2.0)), 8.0, t.hover);
                    }
                    let tile = Rect::from_min_size(pos2(r.min.x + 14.0, r.center().y - 20.0), vec2(40.0, 40.0));
                    let tint = match e.kind {
                        Kind::Folder => Color32::from_rgb(255, 178, 63),
                        Kind::Video => t.accent,
                        Kind::Audio => Color32::from_rgb(64, 190, 120),
                        Kind::Image => Color32::from_rgb(90, 150, 240),
                        _ => t.text_dim,
                    };
                    ui.painter().rect_filled(tile, 9.0, tint.gamma_multiply(0.22));
                    icons::paint(ui.painter(), tile.shrink(10.0), e.kind.icon(), tint);
                    let mut job = egui::text::LayoutJob::simple_singleline(e.name.clone(), Tokens::ui(15.0), t.text);
                    job.wrap.max_width = r.width() - 120.0;
                    job.wrap.max_rows = 1;
                    let g = ui.painter().layout_job(job);
                    ui.painter().galley(pos2(tile.max.x + 12.0, r.center().y - 18.0), g, t.text);
                    let sub = if e.kind == Kind::Folder { String::new() } else { human_size(e.size) };
                    ui.painter().text(pos2(tile.max.x + 12.0, r.center().y + 10.0), Align2::LEFT_CENTER, sub, Tokens::ui(12.0), t.text_dim);
                    if e.kind == Kind::Folder {
                        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.max.x - 24.0, r.center().y), vec2(14.0, 14.0)), Icon::ChevronRight, t.text_dim);
                    } else if multiple(&a.dialog) {
                        let c = Rect::from_center_size(pos2(r.max.x - 26.0, r.center().y), vec2(24.0, 24.0));
                        if chosen {
                            ui.painter().circle_filled(c.center(), 12.0, t.accent);
                            ui.painter().text(c.center(), Align2::CENTER_CENTER, "✓", Tokens::semibold(14.0), Color32::WHITE);
                        } else {
                            ui.painter().circle_stroke(c.center(), 11.0, Stroke::new(1.5, t.text_dim));
                        }
                    }
                    if resp.clicked() {
                        if e.kind == Kind::Folder {
                            go = Some(e.path.clone());
                        } else if is_save(&a.dialog) {
                            a.name = e.name.clone();
                        } else if multiple(&a.dialog) {
                            if chosen {
                                a.chosen.retain(|c| c != &e.path);
                            } else {
                                a.chosen.push(e.path.clone());
                            }
                        } else if !wants_folder(&a.dialog) {
                            finish = Some(vec![e.path.clone()]);
                        }
                    }
                }
            });
        });
    });
    if let Some(m) = app.mobile.as_mut() {
        if let Some(dir) = go
            && let Some(a) = m.picker.active.as_mut()
        {
            a.dir = dir;
            refresh(a);
        }
        if let Some(paths) = finish {
            m.picker.finish(paths);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_kinds_by_extension() {
        assert_eq!(Kind::of(Path::new("/x/VID_2026.MP4")), Kind::Video);
        assert_eq!(Kind::of(Path::new("a.m4a")), Kind::Audio);
        assert_eq!(Kind::of(Path::new("a.HEIC")), Kind::Image);
        assert_eq!(Kind::of(Path::new("edit.fcproj")), Kind::Project);
        assert_eq!(Kind::of(Path::new("notes")), Kind::Other);
    }

    #[test]
    fn listing_filters_by_extension_and_puts_folders_first() {
        let dir = std::env::temp_dir().join(format!("framesmith-picker-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Sub")).unwrap();
        std::fs::write(dir.join("a.mp4"), b"x").unwrap();
        std::fs::write(dir.join("b.txt"), b"x").unwrap();
        std::fs::write(dir.join(".hidden.mp4"), b"x").unwrap();
        let e = list(&dir, &["mp4".into()], false).unwrap();
        let names: Vec<_> = e.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["Sub", "a.mp4"]);
        let folders = list(&dir, &[], true).unwrap();
        assert_eq!(folders.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_dialog_is_answered_through_the_hook() {
        let mut picker = Picker::default();
        let mut run = hook(picker.queue.clone());
        let rx = run(FileDialog::Folder { at: Some(std::env::temp_dir().to_string_lossy().to_string()) });
        picker.take_next();
        assert!(picker.open());
        picker.finish(vec![PathBuf::from("/tmp")]);
        assert_eq!(rx.recv().unwrap(), vec!["/tmp".to_string()]);
        assert!(!picker.open());
        let rx = run(FileDialog::OpenFiles { exts: vec!["mp4".into()] });
        picker.take_next();
        picker.cancel();
        assert!(rx.recv().unwrap().is_empty(), "cancel answers with no paths");
    }
}
