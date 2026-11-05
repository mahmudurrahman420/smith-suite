# Texsmith app icon

**Creature:** an engraved zebra, Texsmith's mascot.

**Style:** an engraving-style portrait in the Crafting Apps owl template (the Framesmith owl): a tight
head-and-shoulders portrait on a full-bleed field, no frame or roundel, with the body running off the tile.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and contour |
| Paper | `#efe9dc` | the figure |
| Texsmith copper | `#c2672b` | the full-bleed field; also the default UI accent "Copper" (`crates/ui-egui/src/theme.rs`) |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, clipped. The macOS `.icns` adds
Apple's transparent margin (tile = 824/1024 of the canvas); Windows and Linux use the tile edge to edge.

**Provenance:** the owner's original Smith Suite drawing (2048 px), vectorised with
craftrules `assets/logo-options/_tools/vectorize_tile.py`. The drawing is kept in craftrules at
`craftrules/assets/app-icons/texsmith/source.png`, not here. `texsmith.svg` is the canonical
artwork. Licence: see `LICENSE.txt`.

## Files

| File | What | Used by |
|---|---|---|
| `texsmith.svg` | master vector (traced at 2048 px, about 840 KB) | source for everything below |
| `texsmith-small.svg` | lighter vector (traced at 1024 px, about 240 KB) | `hicolor/scalable` |
| `texsmith-1024.png` | full tile, 1024 px | docs, store listings |
| `texsmith.icns` | macOS icon set, 16 to 1024 px | `packaging/macos/package.sh` (`CFBundleIconFile`) |
| `texsmith.ico` | Windows icon, 16 to 256 px | `apps/texsmith/build.rs` (embedded in the `.exe`), the MSI (`packaging/windows/package.ps1`) |
| `hicolor/<size>/apps/io.github.mahmudurrahman420.texsmith.png` | Linux theme icons, 16 to 512 px; the 256 one is also the runtime window and taskbar icon (`apps/texsmith/src/main.rs`) and the README icon, and the 16 to 64 px ones the icon at the left of the title bar (`crates/ui-egui/src/titlebar.rs`) | `packaging/linux/io.github.mahmudurrahman420.texsmith.desktop` |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.texsmith.svg` | Linux scalable icon | as above |

## How it reaches each OS

- **macOS:** the app sets the Dock / app-switcher icon at runtime (`ViewportBuilder::with_icon`); the
  `.app` from `packaging/macos/package.sh` carries `texsmith.icns`.
- **Windows:** `build.rs` embeds `texsmith.ico` and VERSIONINFO with `winresource` (taskbar, Start
  menu, Explorer, Alt-Tab); the runtime icon covers the window and taskbar, and the MSI uses the
  same `.ico` for its shortcuts.
- **Linux and FreeBSD:** the packages install `packaging/linux/io.github.mahmudurrahman420.texsmith.desktop` to
  `share/applications/` and `hicolor/` to `share/icons/`; the AppImage and Flatpak carry the same
  files.

## Regenerate

Edit or replace `texsmith.svg` (and `texsmith-small.svg`), then run `packaging/icons.sh`
(needs `resvg`). It writes every PNG, the `.ico` (via `cargo xtask ico`) and the `.icns` (via
`cargo xtask icns`).

To trace a new drawing, use craftrules' vectoriser as the other Crafting Apps did (needs numpy,
pillow and potracer):

```sh
OPT=0.4 DP=1 python3 vectorize_tile.py source.png '#c2672b' 2048 2 texsmith.svg
OPT=1.0 DP=0 python3 vectorize_tile.py source.png '#c2672b' 1024 4 texsmith-small.svg
```
