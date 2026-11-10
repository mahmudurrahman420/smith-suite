//! Framesmith for Android.
//!
//! `libframesmith.so` is started by Android's `NativeActivity` (no Java code): [`android_main`]
//! sets up the session like the desktop app does (auto-save and crash recovery in the app's private
//! storage, the GPU compositor, cpal audio through AAudio) and runs the egui UI in its phone layout
//! ([`filmcraft_ui_egui::mobile`]), in portrait and landscape.
//!
//! On other targets this crate is empty, so the workspace builds and tests everywhere.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::android_main;

/// Where Framesmith keeps projects on the phone's shared storage, under its root.
pub const PROJECTS_DIR: &str = "Documents/Framesmith";
/// Where exports go by default, under the shared storage root.
pub const EXPORTS_DIR: &str = "Movies/Framesmith";

/// The runtime permissions to ask for on an Android API level: reading the user's videos,
/// pictures and music (one permission before Android 13, one per kind after), writing exports on
/// old versions, and the microphone for voice-over.
pub fn permissions_for(sdk: i32) -> Vec<&'static str> {
    let mut p = Vec::new();
    if sdk >= 33 {
        p.extend(["android.permission.READ_MEDIA_VIDEO", "android.permission.READ_MEDIA_IMAGES", "android.permission.READ_MEDIA_AUDIO"]);
    } else {
        p.push("android.permission.READ_EXTERNAL_STORAGE");
    }
    if sdk <= 28 {
        p.push("android.permission.WRITE_EXTERNAL_STORAGE");
    }
    p.push("android.permission.RECORD_AUDIO");
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissions_follow_the_api_level() {
        assert!(permissions_for(34).contains(&"android.permission.READ_MEDIA_VIDEO"));
        assert!(!permissions_for(34).contains(&"android.permission.READ_EXTERNAL_STORAGE"));
        assert!(permissions_for(30).contains(&"android.permission.READ_EXTERNAL_STORAGE"));
        assert!(permissions_for(28).contains(&"android.permission.WRITE_EXTERNAL_STORAGE"));
        assert!(!permissions_for(29).contains(&"android.permission.WRITE_EXTERNAL_STORAGE"));
        assert!(permissions_for(24).contains(&"android.permission.RECORD_AUDIO"));
    }
}
