<p align="center"><img src="logo.svg" alt="Smith Suite" width="360"></p>

# Smith Suite

Open-source creative and productivity apps, written in pure Rust. Each folder is a complete app with its own
Cargo workspace, docs and licence notices.

| App | What it does |
|---|---|
| [Pixelsmith](pixelsmith/) | Image editing: layers, masks, type, PSD files |
| [Pathsmith](pathsmith/) | Vector illustration |
| [Rawsmith](rawsmith/) | Photo library and raw development |
| [Pagesmith](pagesmith/) | Reading, organizing and protecting PDFs |
| [Motionsmith](motionsmith/) | Motion graphics and visual effects |
| [Layoutsmith](layoutsmith/) | Page layout and publishing |
| [Mocksmith](mocksmith/) | Interface design and prototyping |
| [Wordsmith](wordsmith/) | Word processing and documents |
| [Soundsmith](soundsmith/) | Audio recording and editing |
| [Slidesmith](slidesmith/) | Presentations and slide decks |
| [CADsmith](cadsmith/) | Computer-aided design |
| [Cellsmith](cellsmith/) | Spreadsheets and data |
| [Texsmith](texsmith/) | 3D texture painting |
| [Anysmith](anysmith/) | See how any software works, then build your own |
| [Solidsmith](solidsmith/) | Parametric 3D CAD |
| [Frame Smith](framesmith/) | Video editing: timeline, colour, effects and export |

## Download for Windows

The supported way to install the suite is [Smith Creative Cloud](launcher/), one launcher that signs
you in and installs or updates any of the apps. See [launcher/RELEASING.md](launcher/RELEASING.md)
for how a tag becomes an update.

You can still build a single app by hand: open **Actions → Build Windows app → Run workflow**, choose
an app (or `all`, which also builds the launcher), and download the `.exe` from the finished run's
**Artifacts**. Builds are unsigned, so Windows may show "Windows protected your PC": click
**More info → Run anyway**.

## Build from source

Needs [Rust](https://rustup.rs) 1.95 or newer (on Windows, also Visual Studio Build Tools with
"Desktop development with C++").

```sh
git clone https://github.com/mahmudurrahman420/smith-suite.git
cd smith-suite/pixelsmith          # or any other app folder
cargo run --release -p pixelsmith
```

## Licence

MIT OR Apache-2.0, at your option ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)).
Each app folder has its own `NOTICE` with the copyright notices that must be kept with it.
