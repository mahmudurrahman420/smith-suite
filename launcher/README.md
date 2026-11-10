# Smith Creative Cloud

The Windows launcher for the Smith Suite. People download this one program from slmptech.com, sign in
with Google, and install or update any of the 15 apps. It is original Smith Suite software and is not
affiliated with Adobe.

## Run it

```sh
cd launcher
cargo run -p smithcc -- --mock
```

| Environment | How | API |
|---|---|---|
| Production (default) | no flag | `https://slmptech.com/api/v1` |
| Staging | `--env staging` | `https://smith-cc-api-staging.mahmudur76.workers.dev/api/v1` |
| Mock | `--mock` or `--env mock` | a local server inside the process |

`--api-base` overrides the URL. It does not turn on the staging test sign-in. That call
(`POST /dev/sessions/{id}/approve`) is sent only when the selected server is the staging host above.

On Windows, production and staging keep the refresh token in Credential Manager
(service `SmithCreativeCloud`, account `refresh-token-prod` or `refresh-token-staging`). Mock mode,
and every non-Windows build, keep it in a `0600` file under the data directory so tests stay
hermetic.

Data lives in `%LOCALAPPDATA%\SmithSuite\` (`SMITH_CC_HOME` overrides that):

```text
SmithSuite/<app>/          installed app
SmithSuite/shortcuts/      only when SMITH_CC_HOME is set; otherwise the Start Menu
SmithSuite/_launcher/      device id, catalog cache, offline queue, logs
```

## What it does

Sign-in opens the browser. The launcher polls until the backend returns tokens, writes the refresh
token before it uses the access token, and never sends a refresh token again after a completed
refresh call. A connect failure (the server never saw the request) is the only case that keeps the
old token.

The window lists all 15 apps. Install checks sha256, then unpacks a zip or copies a single exe into
the app folder and adds a Start Menu shortcut. Update and uninstall are the same folder. Catalog
checks run at startup and every 6 hours. A 503 `catalog_unavailable` shows "catalog not available
yet" and retries. Update badges and a Windows balloon say when an installed app or the launcher
itself is newer. Self-update downloads `SmithCreativeCloud.exe`, checks its hash, and swaps it on
restart.

Heartbeats and install events queue in `_launcher/outbox.json` when the network is down. Each event
keeps one `Idempotency-Key` across retries. `{ok:true, duplicate:true}` counts as success.

## Tests

```sh
cargo test --workspace --lib
cargo test -p smithcc --lib staging -- --ignored
```

The ignored test signs in against staging with the dev approve endpoint, then checks health, the
catalog 503, heartbeat, install events, device mismatch, refresh, and logout.

## Layout

```text
crates/catalog/     catalog.json types and the GitHub Release assembler
crates/smithcc/     the launcher (library + SmithCreativeCloud binary)
xtask/              cargo run -p xtask -- catalog
catalog/apps.toml   names, taglines, and icon paths for the 15 apps
packaging/windows/  per-user NSIS installer
```

Release steps are in [RELEASING.md](RELEASING.md).
