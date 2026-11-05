//! Windows only: embed the app icon and version info into `cadsmith.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/cadsmith.ico");
    println!("cargo:rerun-if-env-changed=CADSMITH_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADSMITH_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/cadsmith.ico").exists() {
        res.set_icon("../../assets/app-icon/cadsmith.ico");
    }
    res.set("ProductName", "CADsmith")
        .set("FileDescription", "CADsmith computer-aided design")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) 2026 Shohan (mahmudurrahman420) and the CADsmith contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "cadsmith.exe")
        .set("InternalName", "cadsmith");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADSMITH_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=cadsmith.exe built without icon/version resources: {e}");
    }
}
