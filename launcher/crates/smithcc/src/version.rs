use semver::Version;

pub fn is_newer(latest: &str, installed: &str) -> bool {
    match (Version::parse(latest), Version::parse(installed)) {
        (Ok(latest), Ok(installed)) => latest > installed,
        (Ok(_), Err(_)) => true,
        _ => false,
    }
}

pub fn launcher_too_old(current: &str, minimum: &str) -> bool {
    match (Version::parse(current), Version::parse(minimum)) {
        (Ok(current), Ok(minimum)) => current < minimum,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_order() {
        assert!(is_newer("1.2.0", "1.1.0"));
        assert!(!is_newer("1.2.0", "1.2.0"));
        assert!(!is_newer("1.2.0", "1.3.0"));
        assert!(launcher_too_old("1.0.0", "1.2.0"));
        assert!(!launcher_too_old("1.2.0", "1.0.0"));
        assert!(!launcher_too_old("1.0.0", "not-a-version"));
    }
}
