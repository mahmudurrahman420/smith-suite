//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Motionsmith";
pub const APP_SLUG: &str = "motionsmith";
pub const DISCORD: &str = "https://github.com/mahmudurrahman420/motionsmith/discussions";
pub const WEBSITE: &str = "https://github.com/mahmudurrahman420/";
pub const APP_PAGE: &str = "https://github.com/mahmudurrahman420/motionsmith";
pub const GITHUB: &str = "https://github.com/mahmudurrahman420/motionsmith";
pub const ISSUES: &str = "https://github.com/storytold/effectcraft/issues";

/// Sibling apps of the Smith Suite family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("Pixelsmith", "pixelsmith"),
    ("Pathsmith", "pathsmith"),
    ("Framesmith", "framesmith"),
    ("Rawsmith", "rawsmith"),
    ("Pagesmith", "pagesmith"),
    ("Layoutsmith", "layoutsmith"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://github.com/mahmudurrahman420/apps/{slug}")
}
