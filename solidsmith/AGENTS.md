# Solidsmith: instructions for agents

Solidsmith is a clean-room, pure-Rust parametric 3D CAD app (sketch solver, feature timeline, B-rep kernel, an egui UI that agents drive). This file is a router: the hard rules, then the skill to read for your task. Humans: CONTRIBUTING.md.

## Start here
Work: ROADMAP.md, "Where help is wanted most" in CONTRIBUTING.md, issues labelled `help wanted`, `good first issue`, `needs-triage`; architecture in `docs/architecture.md`. Local `plan/` notes are optional.

## Hard rules (what enforces each)
- Independent: never copy another product's code, icons, text or templates; never drive or take data from commercial CAD. `skills/solidsmith-ground-rules/SKILL.md`.
- No GPL/LGPL/AGPL code, not even reading it (OpenCASCADE, FreeCAD, SolveSpace, CadQuery, LibreCAD, QCAD): `cargo xtask licences --check`.
- Every asset is original or openly licensed with a row in ATTRIBUTION.md; read `assets/AGENTS.md` before icon work: `cargo xtask assets`, `cargo xtask icons`.
- Never crash: no unwrap/expect/panic/unsafe in non-test code, errors are `Result`, input is hostile: clippy `-D warnings`, `hostile_params_never_panic`.
- Model decisions (what an operation does, which bodies it touches, which face or edge a point or name means, depths, nearness) use exact geometry; a mesh is for display, export, previews and mesh bodies (`xtask/mesh-decisions.txt` only shrinks): `cargo xtask mesh-decisions`.
- New kernel cases go to solidcore: `cargo xtask lists`.
- Everything is a command and programmatic calls never open dialogs: `skills/solidsmith-commands/SKILL.md`.
- Layering (geom, sketch/kernel/render, doc, io, engine, ui-egui, apps); only `crates/kernel` uses truck: `cargo xtask layers`.
- One outer UI container, one camera path (`navigation::handle`), colours from theme tokens: `cargo xtask ui-containers`.
- Application code is Rust only and builds for wasm: `cargo xtask wasm`, `cargo xtask web-offline-test`.
- The product name is Solidsmith; the old name lives only in `docs/compatibility.md`.
- Sorted lists (message catalogs, `mod` lines, languages) stay sorted: `cargo xtask lists`. Never edit generated files (`docs/features.md`, `THIRD-PARTY-LICENSES.txt`) in a PR.
- A design-format change is ONE new file in `crates/doc/src/format/pending/`: `scripts/test-format-pending.sh`.
- A command has input and output schemas (will become required; `schema/unconverted.txt` only shrinks): `skills/solidsmith-commands/SKILL.md`.
- New UI strings go in `assets/locales/pending/<slug>.json`, ATTRIBUTION rows in `attribution/pending/<slug>.md`: `cargo xtask lists`.
- A changed default keeps the old behaviour behind a preference and fills the PR's `Changes a default:` line: `pr-checks.yml`.
- A regression test goes in the area's existing test file, never a per-bug file (#3226).
- Look at open PRs first, claim with a draft PR (lowest number wins), one topic per PR: `skills/solidsmith-contributing/SKILL.md`.
- A PR's title and first sentence make sense to someone who never saw your chat or the issue: `skills/solidsmith-contributing/SKILL.md`.
- Code, comments, commit messages and docs are in English, always: `skills/solidsmith-contributing/SKILL.md` ("Working as an agent").
- Independent work runs in parallel, dependent work stays sequential: `skills/solidsmith-contributing/SKILL.md` ("Working as an agent").
- Show observable changes with real screenshots or a recording, captured yourself.
- Run `cargo xtask ci` before every commit. `cargo xtask skills` keeps this file and `skills/` honest.

## Working on X: read the skill first
| Working on | Read |
|---|---|
| A PR, a claim, review, CI or evidence | `skills/solidsmith-contributing/SKILL.md` |
| Licences, assets, clean-room, the name | `skills/solidsmith-ground-rules/SKILL.md` |
| Non-test Rust, hostile input, a crash report | `skills/solidsmith-crash-safety/SKILL.md` |
| A command, a crate boundary | `skills/solidsmith-commands/SKILL.md` |
| A dialog, panel, camera, icon, colour | `skills/solidsmith-ui/SKILL.md` |
| Labels, messages, translations | `skills/solidsmith-ui-text/SKILL.md` |
| Saving something new in a design, import, export | `skills/solidsmith-file-formats/SKILL.md` |
| Preferences, a changed default | `skills/solidsmith-preferences/SKILL.md` |
| Writing or running tests | `skills/solidsmith-tests/SKILL.md` |
| Slow work, the window thread, wasm | `skills/solidsmith-performance/SKILL.md` |
| Releases, installers, packages | `skills/solidsmith-packaging/SKILL.md` |
| Running the app, control channel, MCP tools | `skills/solidsmith-agent-tools/SKILL.md` |
| Building a part over MCP | `skills/solidsmith-modeling/SKILL.md` |
| Changing an existing design | `skills/solidsmith-editing/SKILL.md` |
| A bug report, a crash, a wrong result | `skills/solidsmith-reporting/SKILL.md` |
| Checking, porting, designs in git | `skills/solidsmith-checking/SKILL.md` |
| Assemblies, joints, motion | `skills/solidsmith-mechanisms/SKILL.md` |
