# Pathsmith app icon

**Creature:** a regal engraved dragon, Pathsmith's mascot.

**Style:** an engraving-style portrait in the Crafting Apps owl template (the Framesmith owl): a tight
head-and-shoulders portrait on a full-bleed field, no frame or roundel, with the body running off the tile.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and contour |
| Paper | `#efe9dc` | the figure |
| Pathsmith colour | `#e8573f` | the full-bleed field |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, clipped. The macOS files add Apple's
transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** the owner's original Smith Suite drawing, vectorised. `pathsmith.svg` is the canonical
artwork. Licence: see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `pathsmith.svg` | master vector (traced at 2048 px, about 700 KB) | source for everything below |
| `pathsmith-small.svg` | lighter vector (traced at 1024 px, about 370 KB) | `hicolor/scalable` |
| `pathsmith-1024.png` | full tile, 1024 px | docs, store listings |
| `pathsmith-macos-512.png` | tile with Apple margin, 512 px | runtime Dock icon (`apps/pathsmith/src/main.rs`) |
| `pathsmith.icns` | macOS icon set, 16 to 1024 px | `cargo xtask bundle` (`CFBundleIconFile`) |
| `pathsmith.ico` | Windows icon, 16 to 256 px | `apps/pathsmith/build.rs` (embedded in the `.exe`) |
| `hicolor/<size>/apps/io.github.mahmudurrahman420.pathsmith.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime icon on Windows and Linux, the 128 one the brand mark in the app bar and About box (`crates/ui-egui/src/brand.rs`) | `packaging/linux/io.github.mahmudurrahman420.pathsmith.desktop` |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.pathsmith.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `cargo xtask bundle` carries `pathsmith.icns`.
- **Windows:** `build.rs` embeds `pathsmith.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the title bar.
- **Linux:** install `packaging/linux/io.github.mahmudurrahman420.pathsmith.desktop` to `share/applications/` and
  `hicolor/` to `share/icons/`. The app sets its Wayland app ID / X11 class to
  `io.github.mahmudurrahman420.pathsmith`, so the dock matches the window to the launcher.

## Regenerate

Edit or replace `pathsmith.svg` (and `pathsmith-small.svg`), then run `packaging/icons.sh`
(needs `resvg`; `iconutil` on macOS for the `.icns`). It writes every PNG, the `.ico` (via
`cargo xtask ico`) and the `.icns`.
