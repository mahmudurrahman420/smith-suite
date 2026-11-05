# Layoutsmith app icon

**A calico cat wearing a polka-dot scarf**, as an engraving-style head-and-shoulders portrait in the
Crafting Apps' owl-template framing: a full-bleed colour field, the cat in three-quarter view, its
shoulders running off the bottom edge.

## Palette

Exactly three colours:

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | Line work and contour |
| Paper | `#efe9dc` | The figure (the cat and its scarf) |
| Layoutsmith green (app colour) | `#7bb51c` | The full-bleed background field |

## Geometry

A 512 × 512 viewBox, clipped to a rounded square with `rx=112`. Windows and Linux use the full tile. The
macOS icons (`.icns`, `layoutsmith-macos-512.png`) sit on Apple's grid: an 824/1024 body with a
transparent margin.

## Provenance

The owner drew the cat. `layoutsmith.svg` is traced from the drawing at 2048 px and
`layoutsmith-small.svg` at 1024 px.

## Files

| File | Use |
|---|---|
| `layoutsmith.svg` | Canonical artwork (about 680 KB) |
| `layoutsmith-small.svg` | Lighter vector (about 350 KB); also `hicolor/scalable` |
| `layoutsmith-1024.png` | 1024 px render of the full tile |
| `layoutsmith-macos-512.png` | Runtime Dock icon on macOS (embedded by `apps/layoutsmith`) |
| `layoutsmith.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `layoutsmith.ico` | Windows exe icon, 16–256 px (embedded by `apps/layoutsmith/build.rs`) |
| `hicolor/<size>/apps/io.github.mahmudurrahman420.layoutsmith.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.layoutsmith.svg` | Linux scalable icon |
| `LICENSE.txt` | MIT OR Apache-2.0 |

The app ID is `io.github.mahmudurrahman420.layoutsmith`. It's used for the hicolor icon names, the Wayland app ID and
`packaging/linux/io.github.mahmudurrahman420.layoutsmith.desktop`.

## Regenerate

Edit `layoutsmith.svg` (or `layoutsmith-small.svg`), then run `packaging/icons.sh`. It needs `resvg`, and
`iconutil` for the `.icns` (macOS only). It packs the `.ico` with `cargo xtask ico`.
