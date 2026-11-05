# Rawsmith app icon

A **lynx**, frontal head-and-shoulders portrait with an alert gaze, in the style of an engraving: the
Crafting Apps "owl template" (full-bleed colour field, tight portrait, the figure in paper with ink line
work and a crisp ink contour, no roundel or border).

## Palette (exactly three colours)

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and the contour |
| Paper | `#efe9dc` | the figure (the lynx) |
| Rawsmith amber | `#f2a516` | the full-bleed field (the app colour) |

## Geometry

`viewBox="0 0 512 512"`, a rounded-square tile with `rx=112`, clipped; the colour fills the whole tile.
macOS files (`.icns`, `rawsmith-macos-512.png`) add Apple's transparent margin (an 824/1024 body);
Windows and Linux files use the full-bleed tile so the lynx reads at 16-48 px.

## Provenance

The owner's original drawing, made (2880 px, keyed to the three colours) and
vectorised. `rawsmith.svg` here is the canonical artwork. Licence:
`LICENSE.txt` (MIT OR Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `rawsmith.svg` | canonical vector, traced at 2048 px (about 770 KB) |
| `rawsmith-small.svg` | lighter vector, traced at 1024 px (about 360 KB) |
| `rawsmith-1024.png` | 1024 px render (store listings, docs) |
| `rawsmith-macos-512.png` | runtime window/Dock icon on macOS (embedded by `apps/rawsmith/src/main.rs`) |
| `rawsmith.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `rawsmith.ico` | Windows icon, 16-256 px, embedded in `rawsmith.exe` by `apps/rawsmith/build.rs` |
| `hicolor/<n>x<n>/apps/io.github.mahmudurrahman420.rawsmith.png` | Linux icon theme, 16-512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.rawsmith.svg` | Linux scalable icon (the small SVG) |

The app id is `io.github.mahmudurrahman420.rawsmith`: the Wayland app id, the `.desktop` file
(`packaging/linux/io.github.mahmudurrahman420.rawsmith.desktop`, `Icon=io.github.mahmudurrahman420.rawsmith`) and the hicolor icon
name. To install on Linux, copy `hicolor/` into `/usr/share/icons/hicolor/` (or `~/.local/share/icons/hicolor/`)
and the `.desktop` file into `applications/`.

## Regenerate

Edit `rawsmith.svg` (or `rawsmith-small.svg`), then run `packaging/icons.sh` (needs `resvg`; `iconutil`
on macOS for the `.icns`; the `.ico` is packed by `cargo xtask ico`). Every output is committed.
