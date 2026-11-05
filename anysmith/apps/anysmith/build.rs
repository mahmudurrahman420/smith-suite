//! Windows only: embed the app icon and version info (VERSIONINFO) into `anysmith.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `ANYSMITH_REQUIRE_WINRES=1` (set by
//! the release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/anysmith.ico");
    println!("cargo:rerun-if-env-changed=ANYSMITH_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/anysmith.ico")
        .set("ProductName", "Anysmith")
        .set("FileDescription", "Anysmith: find out how any software works")
        .set("LegalCopyright", "Copyright (c) Shohan (mahmudurrahman420) and the Anysmith contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "anysmith.exe")
        .set("InternalName", "anysmith");
    if let Err(e) = res.compile() {
        if std::env::var_os("ANYSMITH_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=anysmith.exe built without icon/version resources: {e}");
    }
}
