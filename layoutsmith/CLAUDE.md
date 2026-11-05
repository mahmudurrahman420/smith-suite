# Layoutsmith — instructions for agents

Layoutsmith is a clean-room, open-source, Rust-native page-layout application targeting Adobe InDesign parity — and superiority (speed, openness, agent control). It runs natively on macOS, Windows and Linux, and on the web via WASM. Siblings with the same conventions: `../pathsmith` (Illustrator-class), `../pixelsmith` (Photoshop), `../pagesmith` (Acrobat), `../framesmith` (Premiere), `../rawsmith` (Lightroom).

## Start every session here
1. Read `plan/STATUS.md` (current milestone, next task), then the task in `plan/execution-plan.md` and the relevant `plan/architecture.md` section. Behaviour reference: `plan/indesign/*.md` (`11-observed-ui.md` holds measured observations of the running app).
2. Follow the autonomous operation protocol (`plan/execution-plan.md` §7). Don't stop to ask unless §7 lists the decision as the user's.

`plan/` is gitignored (local only).

## Never crash
People trust Layoutsmith with their layouts; a crash loses their work. **This outranks feature work**: never ship a feature through a panic path, and fix a crash before building on top of it.
- **No panics in non-test code:** no `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`; no `unsafe` (`unsafe_code = "forbid"`). The one exception is a provably infallible literal (a constant regex, `write!` to a `String`): `#[allow(clippy::expect_used)]` + `.expect("why it cannot fail")`.
- **Errors are `Result<T, E>`** through the crate's error type and `?` (`ok_or(..)?`, `let … else`, `if let`, `map_err` for context). An unfinished feature returns an "unsupported" error. Don't `unwrap_or_default()` where a silent default would corrupt a document; return an error.
- **Input-derived numbers are hostile** (files, commands, MCP/control params, settings): `get()` instead of `[i]`/`[a..b]`, slice strings at char boundaries, checked/saturating arithmetic for lengths and offsets, no division by zero or NaN casts, cap input-sized allocations.
- **Bound recursion** (seen-sets or depth limits for nested or cyclic documents). **Locks:** `lock().unwrap_or_else(PoisonError::into_inner)` or an error; thread joins are `Result`s.
- **Last-resort guard:** escaped panics during command execution and file import/export become an error, never a lost document. It's a safety net, not a licence.
- **Prove it:** every crash fix lands with a small synthetic regression test that panicked before the fix.
- Clean crates carry `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`; new production crate roots must include the attribute before modules or other crate items, with crate-level `//!` documentation allowed before it. Every new workspace crate must also include `[lints] workspace = true` in `Cargo.toml` to inherit `[workspace.lints]`. Tests may `unwrap` (`clippy.toml`).

The Smith Suite name, wordmark and anvil mark in `docs/brand/` are original work by Shohan (mahmudurrahman420), licensed under the project licence (MIT OR Apache-2.0).

## Running and looking at the app
- `cargo run --release -p layoutsmith -- --sample --control 7979` (sample magazine + control channel).
- Drive it: JSON lines on `127.0.0.1:7979`, e.g. `{"id":1,"method":"engine.execute","params":{"command":"frame.create","params":{"rect":[36,36,300,200],"content":"text"}}}` then `{"id":2,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}`. Methods: `crates/ui-egui/src/control.rs`, docs: `docs/control-protocol.md`.
- **For UI work, look at the result** (take `ui.screenshot`, read the PNG) and compare with `plan/indesign/02-ui-ux.md` / `11-observed-ui.md`. If no frame is presented (screen locked) use `ui.render` or `layoutsmith-cli run --sample --all-pages DIR`.
- Headless: `layoutsmith-cli run --sample --cmd 'frame.create={"rect":[0,0,100,100]}' --export out.png`.
- Shell gotcha: `mv`/`cp` are aliased interactive here — use `/bin/mv -f` / `/bin/cp -f`.
- Parallel agents: separate `CARGO_TARGET_DIR` per agent; edit only the crates you own; delete your target dir when done (disk).

## Roadmap
`ROADMAP.md` (committed) is the one-page summary: stage, headline numbers, dimensions, languages, progress log. Details: `docs/target-app-parity.md` (assessment), `docs/gaps.md` (ranked work list: pick from the top), `docs/roadmap.md` (milestones, current focus), `docs/parity-checklist.md` (presence checklist, `cargo xtask parity`), and the typography, file-format, UI, hardware and localization parity docs in `docs/`. When work lands, update the affected docs and their status lines, delete or shrink the closed gap, and add a progress-log line to ROADMAP.md.

## CI workflows
- Workflows in `.github/workflows/` call the shared workflows in [`storytold/craft-common-actions`](https://github.com/storytold/craft-common-actions) at `@main`, with their inputs and secrets. No other jobs: if no shared workflow does what's needed, it doesn't go here.
- Don't add or change CI in a feature or fix PR. If the existing workflows can't build or test a change, link an existing craft-common-actions issue or PR, or open a new issue, or a PR only for a small fix that is safe for every app and does not increase CI time.
- Some workflow files here still run their own jobs. Don't add to them. Moving a job into craft-common-actions is its own task, never part of another PR, and the job stays here until a shared workflow runs it.
