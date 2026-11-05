# Motionsmith app icon

**Creature:** a unicorn, as an engraved head-and-shoulders bust looking out of the tile.

**Style:** an ink engraving portrait in the Crafting Apps' owl-template framing (the Framesmith owl):
full-bleed colour field, tight head-and-shoulders crop, the body running off the bottom and right edges.

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| App colour (Motionsmith pink) | `#e0368f` | the full-bleed field |
| Paper | `#efe9dc` | the figure |
| Ink | `#0b0b0c` | line work and contour |

**Provenance:** the project owner drew it (a 2880 px engraving-style image). It was
vectorised with potrace, which keys out the field and traces the ink.
Licence: see [LICENSE.txt](LICENSE.txt) (MIT OR Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `motionsmith.svg` | master vector, traced at 2048 px (canonical) |
| `motionsmith-small.svg` | lighter vector (~350 KB), traced at 1024 px; also the web favicon |
| `motionsmith-1024.png` | 1024 px render |
| `motionsmith-macos-512.png` | runtime Dock icon on macOS, on Apple's 824/1024 grid |
| `motionsmith.icns` | macOS icon (Apple grid) |
| `motionsmith.ico` | Windows icon, 16–256 px; embedded in `motionsmith.exe` by `apps/motionsmith/build.rs` |
| `hicolor/<n>x<n>/apps/io.github.mahmudurrahman420.motionsmith.png` | Linux icon theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/io.github.mahmudurrahman420.motionsmith.svg` | Linux scalable icon |

App ID: `io.github.mahmudurrahman420.motionsmith` (Wayland app ID, `packaging/linux/io.github.mahmudurrahman420.motionsmith.desktop`).

## Regenerate

After replacing `motionsmith.svg` / `motionsmith-small.svg`, run `packaging/icons.sh` (needs `resvg`; `iconutil`
on macOS for the `.icns`). Copy `motionsmith-small.svg` to `apps/motionsmith-web/web/favicon.svg` too.
