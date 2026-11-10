#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use clap::Parser;

fn main() {
    let cli = smithcc::Cli::parse();
    if let Err(err) = smithcc::launch(cli) {
        eprintln!("Smith Creative Cloud: {err}");
        std::process::exit(1);
    }
}
