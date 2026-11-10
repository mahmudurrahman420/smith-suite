//! Live checks against the staging backend. Ignored in normal `cargo test`
//! because they need the network and a rate-limit budget. Run with:
//! `cargo test -p smithcc --lib staging -- --ignored --nocapture`

use std::thread;
use std::time::Duration;

use crate::api::{ApiClient, InstallEvent, SessionRequest};
use crate::endpoint::{STAGING_API_BASE, dev_approve_allowed};
use crate::paths::Paths;
use crate::store::FileStore;
use crate::vault::Vault;

const DEVICE: &str = "c0ffeec0-0000-4000-8000-00000000cc01";

#[test]
#[ignore = "talks to the staging backend"]
fn staging_contract() {
    assert!(dev_approve_allowed(STAGING_API_BASE));
    let client = ApiClient::new(STAGING_API_BASE, "1.0.0").expect("client");
    let health = client.health().expect("health");
    assert!(health.ok);
    assert_eq!(health.env, "staging");
    eprintln!("staging oauth_configured={}", health.oauth_configured);

    let err = client.catalog().expect_err("catalog");
    assert!(err.is_catalog_unavailable(), "{err}");

    let created = client
        .create_session(&SessionRequest {
            device_id: DEVICE.into(),
            device_name: "SMITHCC-TEST".into(),
            launcher_version: "1.0.0".into(),
            os: "linux-test-x64".into(),
        })
        .expect("session");
    client.approve_dev_session(&created.session_id, Some("smithcc-launcher@example.com"), Some("Launcher Test")).expect("dev approve");
    thread::sleep(Duration::from_secs(created.poll_interval.clamp(1, 5)));
    let issued = match client.poll_session(&created.session_id).expect("poll") {
        crate::api::Poll::Approved(issued) => issued,
        other => panic!("expected approved session, got {other:?}"),
    };
    assert_eq!(issued.user.email, "smithcc-launcher@example.com");
    assert!(issued.user.picture.is_empty());

    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::hermetic(dir.path().to_path_buf());
    let mut vault = Vault::new(Box::new(FileStore::new(paths.token_file(crate::endpoint::Environment::Staging))));
    vault.accept_issued(&issued).expect("save refresh token before use");
    let me = client.me(&issued.access_token).expect("me");
    assert_eq!(me.email, issued.user.email);

    client
        .heartbeat(
            &issued.access_token,
            &crate::api::Heartbeat {
                device_id: DEVICE.into(),
                device_name: "SMITHCC-TEST".into(),
                launcher_version: "1.0.0".into(),
                os: "linux-test-x64".into(),
                installed: vec![],
            },
        )
        .expect("heartbeat");

    let event = InstallEvent {
        device_id: DEVICE.into(),
        app_id: "pixelsmith".into(),
        event: "launch".into(),
        version: "0.0.0".into(),
        from_version: None,
        error: None,
        at: "2026-11-10T00:00:00Z".into(),
    };
    let key = "c0ffee00-0000-4000-8000-0000000000c1";
    let first = client.post_event(&issued.access_token, key, &event).expect("event");
    let second = client.post_event(&issued.access_token, key, &event).expect("duplicate");
    assert!(first.ok);
    assert!(second.duplicate, "repeat Idempotency-Key should be a success duplicate");

    let mismatch = client.heartbeat(
        &issued.access_token,
        &crate::api::Heartbeat {
            device_id: "dddddddd-dddd-4ddd-8ddd-dddddddddddd".into(),
            device_name: "OTHER".into(),
            launcher_version: "1.0.0".into(),
            os: "linux-test-x64".into(),
            installed: vec![],
        },
    );
    assert!(mismatch.expect_err("mismatch").is_device_mismatch());

    let admin = client.admin_me(&issued.access_token);
    assert!(admin.expect_err("not an admin").status() == Some(403));

    vault.rotate(&client, DEVICE).expect("refresh rotates");
    let access = vault.access(&client, DEVICE).expect("new access token");
    assert_eq!(client.me(&access).expect("me after refresh").email, me.email);

    let refresh = std::fs::read_to_string(paths.token_file(crate::endpoint::Environment::Staging)).unwrap();
    let refresh_token = serde_json::from_str::<serde_json::Value>(&refresh).unwrap()["refresh_token"].as_str().unwrap().to_string();
    client.logout(&access, &refresh_token).expect("logout");
    let err = client.refresh(&refresh_token, DEVICE).expect_err("refresh after logout");
    assert!(err.is_unauthorized());
}
