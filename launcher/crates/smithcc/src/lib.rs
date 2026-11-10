//! Smith Creative Cloud, the Windows launcher for the Smith Suite.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

mod api;
mod device;
mod endpoint;
mod error;
mod install;
mod lnk;
mod mock;
mod model;
mod notify;
mod outbox;
mod paths;
mod self_update;
mod state;
mod store;
mod ui;
mod vault;
mod version;
mod worker;

#[cfg(test)]
mod flow;
#[cfg(test)]
mod staging;

use std::path::PathBuf;

use clap::Parser;

use crate::device::Device;
use crate::endpoint::Environment;
use crate::error::{Error, Result};
use crate::mock::MockServer;
use crate::paths::Paths;
use crate::state::Settings;

pub use crate::api::LAUNCHER_VERSION;
pub use crate::endpoint::{Environment as ApiEnvironment, PROD_API_BASE, STAGING_API_BASE};

#[derive(Debug, Parser)]
#[command(name = "Smith Creative Cloud", version, about = "Install and update the Smith Suite")]
pub struct Cli {
    /// `prod` (default), `staging`, or `mock`.
    #[arg(long, env = "SMITH_CC_ENV")]
    pub environment: Option<String>,
    /// Shortcut for `--env mock`.
    #[arg(long, env = "SMITH_CC_MOCK")]
    pub mock: bool,
    /// Override the API base URL. Does not enable the staging test sign-in.
    #[arg(long, env = "SMITH_CC_API_BASE")]
    pub api_base: Option<String>,
    /// Data directory. Defaults to %LOCALAPPDATA%\SmithSuite.
    #[arg(long, env = "SMITH_CC_HOME")]
    pub home: Option<PathBuf>,
    #[arg(long, hide = true)]
    pub apply_self_update: bool,
    #[arg(long, hide = true)]
    pub pid: Option<u32>,
    #[arg(long, hide = true)]
    pub target: Option<PathBuf>,
    #[arg(long, hide = true)]
    pub source: Option<PathBuf>,
}

pub fn launch(cli: Cli) -> Result<()> {
    if cli.apply_self_update {
        let pid = cli.pid.ok_or_else(|| Error::msg("missing --pid"))?;
        let source = cli.source.ok_or_else(|| Error::msg("missing --source"))?;
        let target = cli.target.ok_or_else(|| Error::msg("missing --target"))?;
        return self_update::apply_swap(pid, &source, &target);
    }
    let paths = Paths::resolve(cli.home.as_deref())?;
    std::fs::create_dir_all(paths.state_dir())?;
    let mut settings = Settings::load(&paths)?;
    if cli.mock {
        if matches!(cli.environment.as_deref().and_then(Environment::parse), Some(env) if env != Environment::Mock) {
            return Err(Error::msg("use either --mock or --env, not both"));
        }
        settings.environment = Environment::Mock;
    } else if let Some(name) = &cli.environment {
        settings.environment = Environment::parse(name).ok_or_else(|| Error::msg("environment must be prod, staging, or mock"))?;
    }
    settings.save(&paths)?;

    let base = if let Some(base) = cli.api_base.clone() {
        base
    } else if settings.environment == Environment::Mock {
        let server = MockServer::start()?;
        let url = server.base_url();
        // The local server has to outlive the window. The process exit drops it.
        std::mem::forget(server);
        url
    } else {
        settings.environment.default_base().to_string()
    };

    let device = Device::load_or_create(&paths)?;
    ui::run(ui::Startup { paths, environment: settings.environment, base, device_label: format!("{} · {}", device.name, device.os) })
}
