use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crate::api::{ApiClient, InstallEvent, SessionRequest};
use crate::endpoint::dev_approve_allowed;
use crate::install::{install_package, uninstall_app};
use crate::mock::{MockOptions, MockServer};
use crate::outbox::{FlushStop, Outbox, flush};
use crate::paths::Paths;
use crate::store::FileStore;
use crate::vault::Vault;
use smithcc_catalog::PackageFormat;

#[test]
fn mock_sign_in_catalog_install_and_token_rotation() {
    let server = MockServer::start().unwrap();
    let client = ApiClient::new(&server.base_url(), "1.0.0").unwrap();
    let health = client.health().unwrap();
    assert!(health.ok && health.oauth_configured);
    assert_eq!(health.env, "mock");

    let device = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
    let created = client
        .create_session(&SessionRequest { device_id: device.into(), device_name: "TEST".into(), launcher_version: "1.0.0".into(), os: "linux-test-x64".into() })
        .unwrap();
    let pending = client.poll_session(&created.session_id).unwrap();
    assert!(matches!(pending, crate::api::Poll::Pending));
    client.get_text(&created.login_url).unwrap();
    let issued = match client.poll_session(&created.session_id).unwrap() {
        crate::api::Poll::Approved(issued) => issued,
        other => panic!("expected approval, got {other:?}"),
    };
    assert!(issued.user.picture.is_empty());
    assert!(issued.user.is_admin);

    let dir = tempfile::tempdir().unwrap();
    let paths = Paths::hermetic(dir.path().join("suite"));
    let mut vault = Vault::new(Box::new(FileStore::new(paths.token_file(crate::endpoint::Environment::Mock))));
    vault.accept_issued(&issued).unwrap();
    let me = client.me(&issued.access_token).unwrap();
    assert_eq!(me.email, "ada@gmail.com");

    let catalog = client.catalog().unwrap();
    assert_eq!(catalog.apps.len(), 16);
    let app = &catalog.apps[0];
    assert_eq!(app.id, "pixelsmith");
    let pkg = dir.path().join("pixelsmith.zip");
    let cancel = AtomicBool::new(false);
    client
        .download(crate::api::Download {
            url: &app.package.url,
            dest: &pkg,
            expected_sha256: &app.package.sha256,
            expected_size: app.package.size,
            cancel: &cancel,
            on_progress: &mut |_, _| {},
        })
        .unwrap();
    let entry = install_package(&paths, &app.id, &app.name, PackageFormat::Zip, &pkg, &app.package.entry).unwrap();
    assert!(entry.is_file());
    uninstall_app(&paths, &app.id, &app.name).unwrap();

    let mut outbox = Outbox::default();
    outbox.enqueue_event(InstallEvent {
        device_id: device.into(),
        app_id: "pixelsmith".into(),
        event: "install".into(),
        version: "1.0.0".into(),
        from_version: None,
        error: None,
        at: "2026-11-10T00:00:00Z".into(),
    });
    assert_eq!(flush(&client, &issued.access_token, &mut outbox).unwrap(), FlushStop::Done);
    assert!(outbox.jobs.is_empty());
    assert_eq!(server.event_count(), 1);

    let key = "11111111-2222-4333-8444-555555555555";
    let event = InstallEvent {
        device_id: device.into(),
        app_id: "pixelsmith".into(),
        event: "launch".into(),
        version: "1.0.0".into(),
        from_version: None,
        error: None,
        at: "2026-11-10T00:00:00Z".into(),
    };
    let first = client.post_event(&issued.access_token, key, &event).unwrap();
    let second = client.post_event(&issued.access_token, key, &event).unwrap();
    assert!(!first.duplicate && second.duplicate);
    assert!(client.post_event(&issued.access_token, "  ", &event).is_err());

    let mismatch = client.heartbeat(
        &issued.access_token,
        &crate::api::Heartbeat {
            device_id: "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb".into(),
            device_name: "OTHER".into(),
            launcher_version: "1.0.0".into(),
            os: "linux-test-x64".into(),
            installed: vec![],
        },
    );
    assert!(mismatch.unwrap_err().is_device_mismatch());

    let old = std::fs::read_to_string(paths.token_file(crate::endpoint::Environment::Mock)).unwrap();
    vault.rotate(&client, device).unwrap();
    let saved = std::fs::read_to_string(paths.token_file(crate::endpoint::Environment::Mock)).unwrap();
    assert_ne!(old, saved);
    let access = vault.access(&client, device).unwrap();
    assert_eq!(client.me(&access).unwrap().email, "ada@gmail.com");

    let old_token = serde_json::from_str::<serde_json::Value>(&old).unwrap()["refresh_token"].as_str().unwrap().to_string();
    let err = client.refresh(&old_token, device).unwrap_err();
    assert!(err.is_unauthorized());
    let err = vault.rotate(&client, device).unwrap_err();
    assert!(err.is_unauthorized());

    assert!(!dev_approve_allowed(&server.base_url()));
    let _ = Duration::from_millis(1);
}

#[test]
fn catalog_unavailable_and_session_rate_limit() {
    let server = MockServer::start_with(MockOptions { session_limit: 2, catalog_unavailable: true }).unwrap();
    let client = ApiClient::new(&server.base_url(), "1.0.0").unwrap();
    let err = client.catalog().unwrap_err();
    assert!(err.is_catalog_unavailable());
    assert_eq!(crate::error::backoff_secs(&err, 30), 60);
    let body = SessionRequest {
        device_id: "cccccccc-cccc-4ccc-8ccc-cccccccccccc".into(),
        device_name: "T".into(),
        launcher_version: "1.0.0".into(),
        os: "linux-test-x64".into(),
    };
    client.create_session(&body).unwrap();
    client.create_session(&body).unwrap();
    let err = client.create_session(&body).unwrap_err();
    assert!(err.is_rate_limited());
}
