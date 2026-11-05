# CADsmith — instructions for agents

CADsmith is a clean-room, open-source, pure-Rust computer-aided design and drafting application targeting Autodesk AutoCAD parity — and superiority (speed, openness, agent control). It runs natively on macOS, Windows, Linux and FreeBSD, and on the web via WASM. Siblings with the same conventions: `../pixelsmith` (Photoshop-class), `../pathsmith` (Illustrator), `../framesmith` (Premiere), `../rawsmith` (Lightroom), `../pagesmith` (Acrobat), `../motionsmith` (After Effects), `../layoutsmith` (InDesign).

## Start every session here
1. Read `plan/STATUS.md` (current milestone, next task), then the task in `plan/execution-plan.md` and the relevant `plan/architecture.md` section. Behaviour reference: `plan/autocad/*` (`01-observed-ui.md` holds measured observations of the running reference app; `04-menu-tree.txt` the menu tree by name).
2. Library decisions: `plan/adr/0001-geometry-libraries.md` (what we may depend on, what is rejected for licence reasons).
3. Follow the autonomous operation protocol (`plan/execution-plan.md` §6). Don't stop to ask unless it lists the decision as the owner's.

`plan/` is gitignored (local only).

## Never crash
People trust CADsmith with their drawings; a crash loses their work. **This outranks feature work.**
- **No panics in non-test code:** no `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`; no `unsafe` (`unsafe_code = "forbid"`).
- **Errors are `Result<T, E>`** through the crate's error type and `?`. An unfinished feature returns an error or reports "not available yet"; it never panics.
- **Input-derived numbers are hostile** (DXF/DWG files, command-line text, MCP/control params): `get()` not `[i]`, checked arithmetic, no NaN casts, cap input-sized allocations and loop counts.
- **Bound recursion** (nested/cyclic block references: `MAX_BLOCK_DEPTH`).
- **Last-resort guard:** `Session::execute` and every step of an interactive command (its factory, `begin`, input, `prompt`, `preview` and the passes after it; `crates/engine/src/guard.rs`) run under `catch_unwind`; an escaped panic restores the drawing and reports an error. It does nothing on wasm, where a panic aborts.
- **Prove it:** every crash fix lands with a regression test (see `crates/engine/tests/panic_hunt.rs` and `hostile_dxf_does_not_panic` in `crates/io/src/tests.rs`).
- Every workspace crate manifest carries `[lints] workspace = true` to inherit `[workspace.lints]`; every production crate root carries `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.

The Smith Suite name, wordmark and anvil mark in `docs/brand/` are original work by Shohan (mahmudurrahman420), licensed under the project licence (MIT OR Apache-2.0).

## Running and looking at the app
- `cargo run --release -p cadsmith -- --sample --control PORT` (sample drawing + control channel). Pick a free port; other crafting apps use control ports too.
- Drive it with JSON lines on `127.0.0.1:PORT`:
  - `{"id":1,"method":"cmdline.input","params":{"text":"circle 0,0 5"}}` — type at the command line exactly like a user (prompts, keywords, `@dx,dy`, `@d<a`, direct distance entry).
  - `{"id":2,"method":"engine.execute","params":{"command":"line","params":{"points":[[0,0],[10,0]]}}}` — JSON form.
  - `{"id":3,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}` then read the PNG.
  - Methods: `crates/ui-egui/src/control.rs`; protocol doc: `docs/control-protocol.md`.
- **For UI work, look at the result** (screenshot, read the PNG) and compare with `plan/autocad/screenshots/`. If no frame is presented (screen locked) use `ui.render` or `cadsmith-cli`.
- Headless: `cadsmith-cli run --sample --script 'LINE 0,0 10,10\n' --export out.png`; `cadsmith-cli info file.dxf`; `cadsmith-cli mcp`.
- Shell gotcha: `mv`/`cp` may be aliased interactive — use `/bin/mv -f` / `/bin/cp -f`.
- Parallel agents: separate `CARGO_TARGET_DIR` per agent; edit only the crates you own; delete your target dir when done (disk).

## Roadmap
`ROADMAP.md` (committed) is the one-page summary: stage, parity numbers, estimates. The assessment is `docs/target-app-parity.md`, the work list `docs/gaps.md` (pick work from its top rows), milestones `docs/roadmap.md`, deep checklists `docs/geometry-parity.md`, `docs/ui-parity.md`, `docs/file-format-parity.md`, `docs/hardware-parity.md`, `docs/localization-parity.md`. PRs don't edit these docs: name the affected rows in the PR description, and maintainers update the rows, status lines and revision history in batches, which keeps open PRs from conflicting. `cargo xtask parity` regenerates the menu-breadth checklist in `docs/parity-checklist.md`.

## CI workflows
- Workflows in `.github/workflows/` call the shared workflows in [`storytold/craft-common-actions`](https://github.com/storytold/craft-common-actions) at `@main`, with their inputs and secrets. No other jobs: if no shared workflow does what's needed, it doesn't go here.
- Don't add or change CI in a feature or fix PR. If the existing workflows can't build or test a change, link an existing craft-common-actions issue or PR, or open a new issue, or a PR only for a small fix that is safe for every app and does not increase CI time.
- Some workflow files here still run their own jobs. Don't add to them. Moving a job into craft-common-actions is its own task, never part of another PR, and the job stays here until a shared workflow runs it.
