//! A fully wired [`Session`]: footage decoding through `motionsmith-media` (Framesmith's codecs),
//! the media importer, the expression engine, JavaScript scripting (`motionsmith-script`) and
//! Render Queue export through `motionsmith-export` (Framesmith's encoders). Frontends (desktop,
//! CLI, MCP, web) start here.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::sync::Arc;

use motionsmith_engine::project::render_queue::OutputFormat;
use motionsmith_engine::{ExportJob, ExportResult, Exporter, Importer, Session};
use motionsmith_project::Footage;
pub use motionsmith_script as script;

pub mod app_dirs;

struct MediaImporter;

impl Importer for MediaImporter {
    fn probe(&self, path: &str) -> Result<Footage, String> {
        motionsmith_media::probe(path).map_err(|e| e.to_string())
    }
}

/// Render Queue export via `motionsmith-export`: to the file system, or to `sink` (the web app
/// turns written files into downloads).
#[derive(Default)]
pub struct FileExporter {
    pub sink: Option<Arc<motionsmith_export::Sink>>,
}

impl Exporter for FileExporter {
    fn formats(&self) -> Vec<OutputFormat> {
        motionsmith_export::available_formats()
    }
    fn supports_skip_existing(&self, settings: &motionsmith_project::render_queue::RenderSettings) -> bool {
        // The exporter's existence check precedes storage-overflow redirection.
        self.sink.is_none() && !cfg!(target_arch = "wasm32") && !settings.storage_overflow
    }
    fn export(&self, job: &ExportJob, progress: &mut dyn FnMut(u64, u64) -> bool) -> Result<ExportResult, String> {
        motionsmith_engine::render::passes::block_on(self.export_async(job, progress))
    }

    fn export_async<'a>(
        &'a self,
        job: &'a ExportJob<'a>,
        progress: &'a mut dyn FnMut(u64, u64) -> bool,
    ) -> motionsmith_engine::render_queue::LocalFuture<'a, Result<ExportResult, String>> {
        Box::pin(async move {
            let j = motionsmith_export::Job {
                project: job.project,
                footage: job.footage,
                expr: job.expr,
                accel: job.accel,
                comp: job.item.comp,
                settings: &job.item.settings,
                output: &job.item.output,
                path: job.path,
                sink: self.sink.as_deref(),
                options: motionsmith_export::JobOptions {
                    log: job.item.log,
                    label: job.label.clone(),
                    storage: job.storage,
                    overflow: job.project.render_prefs.overflow_folders.clone(),
                },
                nested_switches: job.nested_switches,
            };
            match motionsmith_export::export_async(&j, &mut |p| progress(p.done, p.total)).await {
                Ok(r) => Ok(ExportResult {
                    path: r.path,
                    frames: r.frames,
                    width: r.width,
                    height: r.height,
                    bytes: r.bytes,
                    seconds: r.seconds,
                    audio: r.audio,
                    log: r.log,
                    overflow: r.overflow,
                }),
                Err(motionsmith_export::ExportError::Cancelled) => Err(motionsmith_engine::render_queue::CANCELLED.into()),
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

/// A new session with media, import, expressions, scripting and export enabled.
pub fn session() -> Session {
    let mut s = Session {
        // The desktop app, the CLI and the MCP server share installed Roto Brush models.
        models_dir: config_dir().map(|d| d.join("models")),
        exporter: Some(Arc::new(FileExporter::default())),
        footage: Arc::new(motionsmith_media::MediaPool::new()),
        importer: Some(Arc::new(MediaImporter)),
        expr: Some(Arc::new(motionsmith_expr::Expressions)),
        expr_check: Some(motionsmith_expr::check_syntax),
        plugin_loader: motionsmith_plugin::wasm_available().then_some(motionsmith_plugin::loader as motionsmith_engine::PluginLoader),
        plugin_previewer: motionsmith_plugin::wasm_available().then_some(motionsmith_plugin::preview as motionsmith_engine::PluginPreviewer),
        plugins_dir: config_dir().map(|d| d.join("Plug-ins")),
        ..Default::default()
    };
    motionsmith_script::install(&mut s);
    s
}

/// The platform config directory for Motionsmith (`MOTIONSMITH_CONFIG_DIR` overrides):
/// `~/Library/Application Support/Motionsmith` (macOS), `%APPDATA%\Motionsmith` (Windows),
/// `$XDG_CONFIG_HOME/motionsmith` or `~/.config/motionsmith` (Linux and others). A `portable.txt`
/// beside the executable puts it in `MotionsmithData` there instead ([`app_dirs`]).
pub fn config_dir() -> Option<std::path::PathBuf> {
    app_dirs::current().dir.clone()
}

#[cfg(test)]
mod tests;
