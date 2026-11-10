//! The Android host: what `apps/filmcraft/src/main.rs` does on the desktop, for a phone.

// The desktop's cpal audio output and microphone input work unchanged on Android (cpal's AAudio
// backend): the same files are compiled here rather than copied.
#[path = "../../../filmcraft/src/audio.rs"]
#[allow(dead_code)]
mod audio;
#[path = "../../../filmcraft/src/audio_in.rs"]
#[allow(dead_code)]
mod audio_in;
mod jni;

use std::path::{Path, PathBuf};

use filmcraft_engine::Session;
use filmcraft_engine::autosave::AutosaveConfig;
use filmcraft_ui_egui::FilmcraftApp;
use filmcraft_ui_egui::mobile::Insets;
use serde_json::json;
use winit::platform::android::activity::{AndroidApp, WindowManagerFlags};

/// The shared storage root when the framework can't be asked.
const SHARED_STORAGE: &str = "/storage/emulated/0";

/// Entry point, called by `android-activity` on the app's main native thread.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag("Framesmith"));
    log::info!("Framesmith {} (android {})", env!("CARGO_PKG_VERSION"), std::env::consts::ARCH);
    let files = app.internal_data_path().unwrap_or_else(|| PathBuf::from("/data/local/tmp/framesmith"));
    let cache = files.parent().map_or_else(|| files.join("cache"), |p| p.join("cache"));
    let shared = jni::shared_storage(&app).unwrap_or_else(|| PathBuf::from(SHARED_STORAGE));
    for d in [&files, &cache] {
        if let Err(e) = std::fs::create_dir_all(d) {
            log::warn!("can't create {}: {e}", d.display());
        }
    }
    set_environment(&shared, &cache, &files);
    // Panics go to <files>/Logs with a backtrace; the UI pass catches them and keeps the session.
    filmcraft_ui_egui::crash::install(Some(files.join("Logs")));
    jni::request_permissions(&app);
    // A video editor keeps the screen on while it is in front (playback, renders, exports).
    app.set_window_flags(WindowManagerFlags::KEEP_SCREEN_ON, WindowManagerFlags::empty());
    register_gpu_frame_renderer();
    let options = eframe::NativeOptions {
        android_app: Some(app.clone()),
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default().with_title("Framesmith"),
        ..Default::default()
    };
    let android = app.clone();
    let result = eframe::run_native(
        "Framesmith",
        options,
        Box::new(move |cc| {
            let mut session = Session::default();
            if let Err(e) = session.start_autosave(AutosaveConfig::new(files.clone())) {
                log::warn!("auto-save and crash recovery unavailable: {e}");
            }
            session.voiceover.input = Some(Box::new(audio_in::CpalIn::new(&session.prefs.audio_hardware.device_class)));
            // Pick up where the user left off; otherwise an empty project to add media to.
            let recent = session.prefs.general.recent_projects.iter().find(|p| Path::new(p).exists()).cloned();
            if let Some(p) = recent
                && let Err(e) = session.execute("file.open", json!({"path": p}))
            {
                log::error!("{e}");
            }
            let mut fapp = FilmcraftApp::new(session);
            if let Some(rs) = cc.wgpu_render_state.clone() {
                fapp.set_wgpu(rs);
            }
            fapp.audio = Some(Box::new(audio::CpalOut::new()));
            fapp.enable_mobile();
            let a = android.clone();
            fapp.hooks.soft_keyboard = Some(Box::new(move |show| {
                if show {
                    a.show_soft_input(true);
                } else {
                    a.hide_soft_input(true);
                }
            }));
            let a = android.clone();
            fapp.hooks.leave = Some(Box::new(move || jni::move_task_to_back(&a)));
            let a = android.clone();
            fapp.hooks.file_written = Some(Box::new(move |path| jni::scan_media(&a, path)));
            let a = android.clone();
            fapp.hooks.open_url = Some(Box::new(move |url| jni::open_url(&a, url)));
            let a = android.clone();
            fapp.hooks.media_access = Some(Box::new(move || jni::request_permissions(&a)));
            Ok(Box::new(Host { app: fapp, android: android.clone(), insets_for: None }))
        }),
    );
    if let Err(e) = result {
        let msg = format!("Framesmith could not start its window: {e}");
        log::error!("{msg}");
        filmcraft_ui_egui::crash::record(&msg);
    }
}

/// Point the engine's folders at the right places on a phone: `HOME` is the shared storage (where
/// the Media Browser and the file picker start, and exports go: `~/Movies`), the temporary
/// directory is the app's cache (Android's default `/data/local/tmp` isn't writable by apps), and
/// the data directory (preferences, auto-save, crash recovery) is the app's private storage.
#[allow(unsafe_code)]
fn set_environment(shared: &Path, cache: &Path, files: &Path) {
    // SAFETY: called first thing in `android_main`, before this process starts any thread of its
    // own that reads the environment; the framework's Java threads don't read these variables.
    unsafe {
        std::env::set_var("HOME", shared);
        std::env::set_var("TMPDIR", cache);
        std::env::set_var("FILMCRAFT_DATA_DIR", files);
    }
    for sub in [crate::PROJECTS_DIR, crate::EXPORTS_DIR] {
        let _ = std::fs::create_dir_all(shared.join(sub));
    }
}

/// The GPU export frame renderer, as on the desktop: exports composite on the GPU and fall back
/// to the CPU reference renderer wherever it cannot.
struct GpuFrameRenderer(filmcraft_gpu::ExportRenderer);

impl filmcraft_export::FrameRenderer for GpuFrameRenderer {
    fn render(
        &mut self,
        project: &filmcraft_project::Project,
        seq: filmcraft_project::ItemId,
        t: filmcraft_time::Tick,
        opts: filmcraft_render::RenderOptions,
        sources: &dyn filmcraft_render::SourceProvider,
    ) -> Option<filmcraft_render::Image> {
        self.0.render(project, seq, t, opts, sources)
    }
}

fn register_gpu_frame_renderer() {
    filmcraft_export::register_frame_renderer(|| {
        filmcraft_gpu::ExportRenderer::new().map(|r| Box::new(GpuFrameRenderer(r)) as Box<dyn filmcraft_export::FrameRenderer>)
    });
}

/// The app as eframe runs it: Framesmith's UI plus what only the Android host knows (the system
/// bars and display cutout to keep clear of).
struct Host {
    app: FilmcraftApp,
    android: AndroidApp,
    /// The screen size the insets were last read for.
    insets_for: Option<egui::Vec2>,
}

impl Host {
    /// Re-read the system bar insets when the screen changes size (rotation, split screen).
    fn refresh_insets(&mut self, ctx: &egui::Context) {
        let size = ctx.content_rect().size();
        if self.insets_for == Some(size) {
            return;
        }
        let Some([top, bottom, left, right]) = jni::system_insets(&self.android) else {
            // the decor view has no insets before its first layout: ask again next frame
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
            return;
        };
        self.insets_for = Some(size);
        let ppp = ctx.pixels_per_point().max(0.1);
        if let Some(m) = self.app.mobile.as_mut() {
            m.insets = Insets { top: top as f32 / ppp, bottom: bottom as f32 / ppp, left: left as f32 / ppp, right: right as f32 / ppp };
        }
    }
}

impl eframe::App for Host {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.app.raw_input_hook(ctx, raw_input);
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.logic(ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.refresh_insets(ui.ctx());
        self.app.ui(ui, frame);
    }

    fn on_exit(&mut self) {
        self.app.on_exit();
    }
}
