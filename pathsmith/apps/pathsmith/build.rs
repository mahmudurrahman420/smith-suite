//! Windows only: embed the app icon and version info (VERSIONINFO) into `pathsmith.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PATHSMITH_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/pathsmith.ico");
    println!("cargo:rerun-if-env-changed=PATHSMITH_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/pathsmith.ico")
        .set("ProductName", "Pathsmith")
        .set("FileDescription", "Pathsmith vector illustration editor")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) the Pathsmith authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pathsmith.exe")
        .set("InternalName", "pathsmith");
    if let Err(e) = res.compile() {
        if std::env::var_os("PATHSMITH_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=pathsmith.exe built without icon/version resources: {e}");
    }
    Ok(())
}
