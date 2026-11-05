//! Windows only: embed the app icon and version info (VERSIONINFO) into `pagesmith.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab. A separate PDF document icon
//! uses resource ID 2; the installer references it as `pagesmith.exe,-2`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PAGESMITH_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

#[path = "src/windows_manifest.rs"]
mod windows_manifest;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/pagesmith.ico");
    println!("cargo:rerun-if-changed=../../assets/app-icon/pagesmith-document.ico");
    println!("cargo:rerun-if-changed=src/windows_manifest.rs");
    println!("cargo:rerun-if-env-changed=PAGESMITH_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/pagesmith.ico")
        .set_icon_with_id("../../assets/app-icon/pagesmith-document.ico", "2")
        .set_manifest(windows_manifest::WINDOWS_MANIFEST)
        .set("ProductName", "Pagesmith")
        .set("FileDescription", "Pagesmith PDF workbench")
        .set("LegalCopyright", "Copyright (c) the Pagesmith contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pagesmith.exe")
        .set("InternalName", "pagesmith");
    if let Err(e) = res.compile() {
        if std::env::var_os("PAGESMITH_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=pagesmith.exe built without icon/version resources: {e}");
    }
}
