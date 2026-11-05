# Contributing

- **Language:** The Smith Suite name, wordmark and anvil mark in `docs/brand/` are original work by Shohan (mahmudurrahman420), licensed under the project licence (MIT OR Apache-2.0).

## Adding a command

1. **Find the id.** Search `crates/ui-egui/src/menu_catalog.rs` for the Photoshop menu item. Using its id makes the menu item live with no UI work. Commands without a Photoshop menu entry use a descriptive id in the same style (`layer.smartFilter.delete`) and an empty menu path.
2. **Algorithm** goes in the lowest crate that fits (`algo` for imaging, `paint`, `vector`, `text`, `cms`), with unit tests. It takes depth-agnostic surfaces (`pixelsmith-raster`), works per tile, and is deterministic (seeded randomness).
3. **Command** goes in an engine module (`crates/engine/src/<area>_cmds.rs`) exposing `specs()`, registered with `v.extend(...)` in `commands.rs`. Fill in:
   - `id`, `label`, `menu` path and Photoshop's default `shortcut`,
   - a params doc string such as `{"radius":px=4,"mode":"a|b"="a"}` (this is what agents read through `commands` / `command_list`),
   - an `enabled` predicate (greys the menu item out),
   - `run`, which returns a JSON result and records exactly **one** history step (use `coalesce` for drags and typing sessions).
4. **Respect the context:** active selection (feathered), layer vs mask target, locks, the colour model and depth.
5. **Tests** in the module: behaviour, undo/redo, disabled states, bad params, several depths.
6. **Dialog** (if it has parameters): filters and adjustments get schema-driven dialogs with live preview (`ui-egui/src/filter_dialog.rs`, `dialogs.rs`).
7. Run `cargo xtask parity` and commit the updated `docs/parity-checklist.md`.
