# Cellsmith app icon (PLACEHOLDER)

**This icon is temporary.** Every Crafting App's icon is an engraved creature portrait that the
product owner makes; Cellsmith's hasn't been made yet.
Until it is, this placeholder stands in: a simple ledger page with a header row, a row-number
column, a selected cell with its fill handle and a double rule under a total, on a full-bleed
Cellsmith-green tile. It is original geometry written by hand for this repository, deliberately
not modelled on any vendor's spreadsheet logo (no letters, no "X", no overlapping panels).

When the owner's art arrives, replace `cellsmith.svg`, run `packaging/icons.sh`, update this
README (provenance, creature) and the rows in `ATTRIBUTION.md`, and drop the word placeholder.

## Palette

Exactly three colours:

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | Line work and contour |
| Paper | `#efe9dc` | The page |
| Cellsmith green (app colour, **proposed**) | `#1f9d55` | The full-bleed field and the header cells |

The app colour is a proposal pending the owner's approval: Cellsmith green `#1f9d55`, with ink
(darker) variant `#147a40` for text and accents on light backgrounds. It differs from
Layoutsmith's `#7bb51c`.

## Geometry

A 512 × 512 viewBox, clipped to a rounded square with `rx=112`. Windows and Linux use the full
tile. The macOS icons (`.icns`, `cellsmith-macos-512.png`) sit on Apple's grid: an 824/1024
body with a transparent margin.

## Files

| File | Use |
|---|---|
| `cellsmith.svg` | Canonical artwork; also `hicolor/scalable` |
| `cellsmith-1024.png` | 1024 px render of the full tile |
| `cellsmith-macos-512.png` | Runtime Dock icon on macOS (for `apps/cellsmith` to embed) |
| `cellsmith.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `cellsmith.ico` | Windows exe icon, 16–256 px (embedded by `apps/cellsmith/build.rs`) |
| `hicolor/<size>/apps/io.github.mahmudurrahman420.cellsmith.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.cellsmith.svg` | Linux scalable icon |
| `LICENSE.txt` | MIT OR Apache-2.0 |

The app ID is `io.github.mahmudurrahman420.cellsmith`. It's used for the hicolor icon names, the Wayland app
ID and `packaging/linux/io.github.mahmudurrahman420.cellsmith.desktop`.

## Regenerate

Edit `cellsmith.svg`, then run `packaging/icons.sh`. It needs `resvg`, and `iconutil` for the
`.icns` (macOS only). It packs the `.ico` with `cargo xtask ico`.
