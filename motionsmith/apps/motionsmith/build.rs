//! Windows only: embed the app icon and version info (VERSIONINFO) into `motionsmith.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `MOTIONSMITH_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/motionsmith.ico");
    println!("cargo:rerun-if-env-changed=MOTIONSMITH_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/motionsmith.ico")
        .set("ProductName", "Motionsmith")
        .set("FileDescription", "Motionsmith motion graphics and visual effects")
        .set("LegalCopyright", "Copyright (c) 2026 The Motionsmith contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "motionsmith.exe")
        .set("InternalName", "motionsmith");
    if let Err(e) = res.compile() {
        if std::env::var_os("MOTIONSMITH_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=motionsmith.exe built without icon/version resources: {e}");
    }
}
