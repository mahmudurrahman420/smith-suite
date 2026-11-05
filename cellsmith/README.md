<p align="center">
  <a href="https://github.com/mahmudurrahman420/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/smithsuite-logo-white.svg">
      <img alt="Smith Suite" src="docs/brand/smithsuite-logo.svg" width="200">
    </picture>
  </a>
</p>

<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/io.github.mahmudurrahman420.cellsmith.png" width="128" height="128" alt="Cellsmith icon: a spreadsheet page on a green tile">
</p>

<h1 align="center">Cellsmith</h1>



<p align="center">
  <b>Spreadsheets and data; an open-source, high-performance modern spreadsheet application, built in pure Rust.</b>
</p>

<p align="center">
  A fast, open-source, clean-room take on modern spreadsheet workflows. It runs natively on macOS,
  Windows, Linux and BSD, and in the browser via WebAssembly.<br>
  <i>By the Smith Suite team.</i>
</p>

<p align="center">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-147a40?style=flat-square&logo=rust&logoColor=white">
  <img alt="Runs on macOS, Windows, Linux, BSD and the web" src="https://img.shields.io/badge/runs%20on-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-1f9d55?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-147a40?style=flat-square">
  <img alt="Agent-drivable over MCP" src="https://img.shields.io/badge/agents-MCP-1f9d55?style=flat-square">
  <br>
  <a href="https://sonarcloud.io/project/overview?id=storytold_cellsmith"><img alt="Scanned by SonarCloud" src="https://img.shields.io/badge/Scanned_by-SonarCloud-4b9fd5?style=flat-square&logo=sonarqubecloud&logoColor=white"></a>
  <a href="https://coderabbit.ai"><img alt="Reviewed by CodeRabbit" src="https://img.shields.io/coderabbit/prs/github/mahmudurrahman420/cellsmith?style=flat-square&label=Reviewed%20by%20CodeRabbit&logo=coderabbit"></a>
</p>

<p align="center">
  <a href="https://github.com/mahmudurrahman420/cellsmith/discussions"><img alt="Join the Smith Suite community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://github.com/mahmudurrahman420/cellsmith"><b>Cellsmith on github.com/mahmudurrahman420</b></a> ·
  <a href="https://github.com/mahmudurrahman420/">Smith Suite</a> ·
  <a href="https://github.com/mahmudurrahman420/apps">All Crafting Apps</a>
</p>

<br>

<p align="center">
  <img src="docs/images/hero-sales.png" alt="Cellsmith showing a quarterly sales dashboard: a styled table with banded rows, filter buttons, a totals row, data bars, arrow icons and sparklines, next to a clustered column chart and a pie chart" width="100%">
  <br><sub><b>Quarterly Sales Dashboard</b>: a table with a totals row, conditional formatting, sparklines and two live charts — built entirely from Cellsmith commands (<code>File → New → Sample</code>).</sub>
</p>

> [!NOTE]
> **Smith Suite is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Discord](https://github.com/mahmudurrahman420/cellsmith/discussions).**

<p align="center">
  <a href="#a-look-around">A look around</a> ·
  <a href="#why-cellsmith">Why Cellsmith</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#for-agents-mcp-cli-and-the-control-channel">For agents</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#the-crafting-apps">The Crafting Apps</a> ·
  <a href="#license-and-credits">License and credits</a>
</p>

## A look around

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/budget.png" alt="A household budget with accounting number formats, red highlights for overspent categories, a green-yellow-red colour scale on the percent-used column, a totals row with a double border, and a horizontal bar chart comparing budget with actual" width="100%"><p align="center"><sub><b>Formatting.</b> Accounting formats, cell styles, conditional rules and colour scales, frozen headers and a bar chart.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/grades-formulas.png" alt="A gradebook table with averages and letter grades computed with IFS, green highlights on A grades and data bars, with the Formulas ribbon tab open showing the function library" width="100%"><p align="center"><sub><b>Formulas.</b> 500+ functions, dynamic arrays, LET and LAMBDA, structured table references.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/dark-insert.png" alt="The sales dashboard in dark mode with the Insert ribbon tab open, showing Pivot Table, Table, Pictures, Shapes, chart, sparkline, link, comment, text box and symbol buttons" width="100%"><p align="center"><sub><b>Dark mode</b> and the Insert tab: tables, charts, sparklines, pictures, shapes, links and comments.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/pivot.png" alt="A pivot table summarising revenue by region and product with a Channel report filter, grand totals and currency formatting, with the Pivot Table Fields pane open on the right showing ticked fields and the Filters, Columns, Rows and Values areas" width="100%"><p align="center"><sub><b>Pivot tables.</b> Field list, filters, layouts, date grouping and show-values-as, saved to and read from XLSX.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/format-chart.png" alt="The sales dashboard with the column chart selected: resize handles on the chart, its source data outlined in the table, the Chart Design tab open and the Format Chart Area pane on the right" width="100%"><p align="center"><sub><b>Charts.</b> Seventeen chart types, Chart Design tab and a Format pane; selecting a chart outlines its source data.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/page-break.png" alt="The household budget in Page Break Preview: the print area outlined in blue with a dashed page break, Page 1 and Page 2 watermarks, and the area outside the print range greyed out" width="100%"><p align="center"><sub><b>Page Break Preview</b>, print settings, and PDF export and printing.</sub></p></td>
</tr>
</table>

## Why Cellsmith

- **Familiar.** Ribbon, formula bar, Name Box, sheet tabs, keyboard shortcuts, fill handle,
  point-mode formula entry with coloured references, Format Cells, Paste Special,
  conditional formatting, tables, charts and more. Everything is right where you expect it.
- **Compatible.** XLSX is the native format: styles, themes, formulas (including dynamic arrays),
  tables, conditional formats, validation, comments, hyperlinks, charts, pictures, sparklines,
  pivot tables and print settings round-trip. CSV and TSV too.
  Legacy 97-2003 `.xls` workbooks open with their values, formulas and number formats (formatting and objects not yet).
  [XLSB worksheet data can be imported](docs/xlsb-import.md), using saved values instead of formulas.
  [ODS worksheet data can be imported](docs/ods-import.md), using saved values instead of formulas.
  [Static pictures can live inside cells](docs/cell-pictures.md), fitting their size and moving with the data.
- **Capable.** A dependency-graph calculation engine with dynamic arrays and spilling,
  500+ worksheet functions plus LET, LAMBDA, MAP, REDUCE, SCAN, BYROW, BYCOL and MAKEARRAY,
  structured table references, a comprehensive number-format language, sort and AutoFilter,
  Flash Fill, data validation, outlining and subtotals.
- **Fast.** Copy-on-write workbooks (undo snapshots are nearly free), incremental recalculation,
  and a virtualised grid that only lays out what's on screen.
- **Agent-native.** Every ribbon button, menu item, dialog, keystroke and mouse gesture is a
  command that agents can run over a JSON control channel, an **MCP server**, or the CLI.
- **Everywhere.** One Rust codebase for desktop and the web. No subscription, no licence server,
  no telemetry.

## Quick start

```sh
cargo run --release -p cellsmith                              # desktop app
cargo run --release -p cellsmith -- --sample sales            # open a sample (sales, budget, grades)
cargo run --release -p cellsmith -- book.xlsx                 # open a workbook
cargo run --release -p cellsmith -- --sample sales --control 7979   # + JSON control channel
cargo xtask ci                                                 # fmt, clippy, tests, assets, layering, wasm
```

Choose Español under View → Interface language for [Spanish formula names, decimal commas, and semicolon arguments](docs/formula-input.md).
Workbooks, clipboard exchange, and programmatic APIs retain canonical English syntax.

### Web

```sh
cd apps/cellsmith-web && trunk build --release     # → dist/web (serve it with any static server)
cd apps/cellsmith-web && trunk serve --release     # http://127.0.0.1:8771  (?sample=sales opens a sample)
```

You need [trunk](https://trunk-rs.github.io/trunk/) and the `wasm32-unknown-unknown` target. Open uses the
browser's file picker (dropping files works too) and Save downloads the workbook.

### Ribbon access keys

Press and release **left Alt** on Windows/Linux/BSD, or press **F10** on any platform,
then type a sequence below one key at a time. Keytips appear on the supported Home
controls and menu items. On macOS, use F10 (Fn+F10 if required by your keyboard); Option
remains available for entering accented characters.

| Sequence after Alt/F10 | Action |
|---|---|
| `H A L` / `H A C` / `H A R` | Align left / center / right |
| `H V S` or legacy `E S` | Open Paste Special |
| `H O I` or legacy `O C A` | AutoFit selected columns |

In Paste Special, **T/V/F/C** select formats/values/formulas/comments, and **E** toggles
Transpose. **Enter** applies the selected options; **Escape** cancels the dialog.
These letters also work with Alt held on Windows/Linux/BSD.

Escape backs out one keytip level; Alt/F10 again exits keytips. Clicking, scrolling,
or switching windows cancels navigation. Access keys do not start while editing text
or using another dialog. This is initial support for these Home and legacy paths;
other ribbon commands and full mouse-free navigation remain to be implemented.
Browsers or desktop environments may reserve Alt/F10 for their own menus.

### Logs

The desktop app writes its log to standard error and to `cellsmith.log` in the `logs` folder
next to its settings: `~/.config/cellsmith/logs` on Linux and BSD (`$XDG_CONFIG_HOME/cellsmith/logs`
when set), `~/Library/Application Support/Cellsmith/logs` on macOS and
`%APPDATA%\Cellsmith\logs` on Windows. Each start moves the previous log to `cellsmith.1.log` and
that one to `cellsmith.2.log`, so the log of a run that crashed survives the next start; attach
them to a bug report. A log file stops growing at 16 MiB. By default Cellsmith's own crates log
at `info` and everything else at `warn`; `RUST_LOG` replaces that with env_logger-style
directives, e.g. `RUST_LOG=debug` or `RUST_LOG=warn,cellsmith_engine=debug` (a trailing `*`
matches a prefix: `cellsmith*=debug`). Runs with `CELLSMITH_NO_PREFS` set (no saved settings)
log to standard error only.

## For agents: MCP, CLI and the control channel

```sh
claude mcp add cellsmith -- cellsmith-cli mcp                 # headless MCP server
cellsmith-cli mcp --connect 7979                               # MCP bridged to a running window
cellsmith-cli eval '=XLOOKUP("b",{"a","b"},{1,2})'             # → 2
cellsmith-cli run --sample budget --cmd 'home.bold={"range":"B4:F4"}' --out budget.xlsx
cellsmith-cli cat budget.xlsx --range B4:F15                   # aligned text table
cellsmith-cli commands --search chart                          # every command and its params
```

The MCP server offers tools such as `read_range`, `write_range`, `set_cell`, `evaluate_formula`,
`format_range`, `insert_chart`, `create_table`, `sort_range`, `filter` and `execute_command`
(which reaches every one of Cellsmith's commands). The desktop app's control channel adds real
pointer and keyboard input, dialogs and screenshots. See [`docs/mcp.md`](docs/mcp.md),
[`docs/cli.md`](docs/cli.md) and [`docs/control-protocol.md`](docs/control-protocol.md).

## Architecture

Cellsmith is an engine-first Cargo workspace with enforced layering (`cargo xtask layers`). The
egui frontend is a separate crate, so the UI can be swapped without touching the engine.

| Layer | Crates |
|---|---|
| L0 | `core` (addresses, values, errors, dates, input parsing) · `numfmt` (number format codes) |
| L1 | `formula` (lexer, parser, printer, reference adjustment) · `functions` (worksheet function library) |
| L2 | `model` (workbook, sheets, copy-on-write cells, styles, tables, charts…) · `calc` (dependency graph, evaluator, dynamic arrays) · `xlsx` (XLSX, CSV, plus XLSB and ODS data import) |
| L3 | `chart` (toolkit-free chart layout and rendering) |
| L5 | `engine` (session, commands, history, clipboard, fill, sort, filter, file I/O) |
| L6 | `ui-egui` (spreadsheet UI, control channel) · `mcp` (MCP server) |
| L7 | `apps/cellsmith`, `apps/cellsmith-cli`, `apps/cellsmith-web` |

- **Contributor and agent rules** (clean-room, asset policy, never-crash, quality gates): [`AGENTS.md`](AGENTS.md)
- **Bundled assets:** each one is listed with its licence in [`ATTRIBUTION.md`](ATTRIBUTION.md)
- **Releases:** signed macOS, Windows, Linux, BSD and web builds; see [`docs/releasing.md`](docs/releasing.md)

## Roadmap

Cellsmith is in **alpha**: the core workflow works end to end, but depth, fidelity and file
compatibility are still rough. It covers 89% of standard ribbon commands in our catalog
([`docs/parity-checklist.md`](docs/parity-checklist.md)) and 513 standard worksheet functions
([`docs/function-parity.md`](docs/function-parity.md)); this is a parity inventory, not a count of working implementations. Weighted by what spreadsheet users rely on, we
estimate it is about 50% ready for real work. Next up: XLSX fidelity on real-world files, very large
workbooks and multi-threaded recalc, chart depth, text in every script, and `.xls` import. Stage,
numbers and estimates are in [ROADMAP.md](ROADMAP.md); the ranked work list is [`docs/gaps.md`](docs/gaps.md).

## Downloads

**Download Cellsmith** from GitHub: the [latest release](https://github.com/mahmudurrahman420/cellsmith/releases/latest) has every build listed below, and [all releases](https://github.com/mahmudurrahman420/cellsmith/releases) has earlier versions and their notes. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `cellsmith-<ver>-windows-x64.msi` | `cellsmith-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `cellsmith-<ver>-windows-arm64.msi` | `cellsmith-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `cellsmith-<ver>-windows-x86.msi` | `cellsmith-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed. The MSI installer offers an optional desktop shortcut
(unchecked on a new installation).

**If the app doesn't open on Windows:** the desktop app initializes only DirectX 12 by default.
Letting wgpu also create an OpenGL instance can crash some graphics drivers (AMD's
`atio6axx.dll`) before the window appears, so the app would flash in Task Manager and quit.
`WGPU_BACKEND` overrides the default for troubleshooting (for example `dx12` or `vulkan`). In
PowerShell, from the folder containing the executable:

```powershell
$env:WGPU_BACKEND = "vulkan"
& .\cellsmith.exe
Remove-Item Env:WGPU_BACKEND                     # restore the default for later launches
```

An explicit `gl` override can bring the driver crash back on affected systems, and `vulkan` can
bring back a crash in some Intel UHD Graphics drivers (`igvk64.dll`, #120). The macOS, Linux and
web backend defaults are unchanged.

**If the app is slow on an older Windows PC, or DirectX 12 can't start:** when DirectX 12 finds no
graphics card with a driver for it, it only offers the Windows-provided software renderer, which draws every
frame on the processor. In that case Cellsmith uses OpenGL instead, without setting anything; if
the first renderer fails to start, it tries the other one once. The log file records the choice
(lines starting with `graphics:`). `WGPU_BACKEND` still overrides it (and then nothing else is
tried), e.g. `$env:WGPU_BACKEND = "gl"` or `"dx12"`.

DirectX 12 shaders are compiled with FXC, which is part of Windows, so no `dxcompiler.dll` or
`dxil.dll` is needed, and a copy another program put on `PATH` can no longer stop the app at start
("Parent device is lost", #339). `$env:WGPU_DX12_COMPILER = "dxc"` (or `"auto"`) selects wgpu's DXC
handling again for troubleshooting.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `cellsmith-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `cellsmith-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `cellsmith-<ver>-linux-x86_64.AppImage` | `cellsmith-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `cellsmith-<ver>-linux-x86_64.flatpak` | `cellsmith-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `cellsmith-<ver>-linux-x86_64.deb` | `cellsmith-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `cellsmith-<ver>-linux-x86_64.rpm` | `cellsmith-<ver>-linux-aarch64.rpm` | |
| Tarball | `cellsmith-<ver>-linux-x86_64.tar.gz` | `cellsmith-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `cellsmith-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `cellsmith-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## The Crafting Apps

Cellsmith is one of the **Crafting Apps**: free, open-source creative tools from the
[Smith Suite](https://github.com/mahmudurrahman420/) team, each written from scratch in Rust and each able to
stand on its own.

| | App | What it's for | Code | Learn more |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/pixelsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.pixelsmith.png" alt="" width="32" height="32"> | **Pixelsmith** | Image editing: layers, masks, type and real PSD files | [GitHub](https://github.com/mahmudurrahman420/pixelsmith) | [Website](https://github.com/mahmudurrahman420/pixelsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/pathsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.pathsmith.png" alt="" width="32" height="32"> | **Pathsmith** | Vector illustration | [GitHub](https://github.com/mahmudurrahman420/pathsmith) | [Website](https://github.com/mahmudurrahman420/pathsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/framesmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.framesmith.png" alt="" width="32" height="32"> | **Framesmith** | Video editing, color and sound | [GitHub](https://github.com/mahmudurrahman420/framesmith) | [Website](https://github.com/mahmudurrahman420/framesmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/rawsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.rawsmith.png" alt="" width="32" height="32"> | **Rawsmith** | Photo library and raw development | [GitHub](https://github.com/mahmudurrahman420/rawsmith) | [Website](https://github.com/mahmudurrahman420/rawsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/pagesmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.pagesmith.png" alt="" width="32" height="32"> | **Pagesmith** | Reading, organizing and protecting PDFs | [GitHub](https://github.com/mahmudurrahman420/pagesmith) | [Website](https://github.com/mahmudurrahman420/pagesmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/motionsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.motionsmith.png" alt="" width="32" height="32"> | **Motionsmith** | Motion graphics and visual effects | [GitHub](https://github.com/mahmudurrahman420/motionsmith) | [Website](https://github.com/mahmudurrahman420/motionsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/layoutsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.layoutsmith.png" alt="" width="32" height="32"> | **Layoutsmith** | Page layout and publishing | [GitHub](https://github.com/mahmudurrahman420/layoutsmith) | [Website](https://github.com/mahmudurrahman420/layoutsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/mocksmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.mocksmith.png" alt="" width="32" height="32"> | **Mocksmith** | Interface design and prototyping | [GitHub](https://github.com/mahmudurrahman420/mocksmith) | [Website](https://github.com/mahmudurrahman420/mocksmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/wordsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.wordsmith.png" alt="" width="32" height="32"> | **Wordsmith** | Word processing and documents | [GitHub](https://github.com/mahmudurrahman420/wordsmith) | [Website](https://github.com/mahmudurrahman420/wordsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/soundsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.soundsmith.png" alt="" width="32" height="32"> | **Soundsmith** | Audio recording and editing | [GitHub](https://github.com/mahmudurrahman420/soundsmith) | [Website](https://github.com/mahmudurrahman420/soundsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/slidesmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.slidesmith.png" alt="" width="32" height="32"> | **Slidesmith** | Presentations and slide decks | [GitHub](https://github.com/mahmudurrahman420/slidesmith) | [Website](https://github.com/mahmudurrahman420/slidesmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/cadsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.cadsmith.png" alt="" width="32" height="32"> | **CADsmith** | Computer-aided design | [GitHub](https://github.com/mahmudurrahman420/cadsmith) | [Website](https://github.com/mahmudurrahman420/cadsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/cellsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.cellsmith.png" alt="" width="32" height="32"> | **Cellsmith** | Spreadsheets and data | [GitHub](https://github.com/mahmudurrahman420/cellsmith) | [Website](https://github.com/mahmudurrahman420/cellsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/texsmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.texsmith.png" alt="" width="32" height="32"> | **Texsmith** | 3D texture painting | [GitHub](https://github.com/mahmudurrahman420/texsmith) | [Website](https://github.com/mahmudurrahman420/texsmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/anysmith/main/assets/app-icon/hicolor/64x64/apps/io.github.mahmudurrahman420.anysmith.png" alt="" width="32" height="32"> | **Anysmith** | See how any software works, then build your own | [GitHub](https://github.com/mahmudurrahman420/anysmith) | [Website](https://github.com/mahmudurrahman420/anysmith) |
| <img src="https://raw.githubusercontent.com/mahmudurrahman420/solidsmith/main/assets/icons/app/linux/exports/hicolor/64x64/apps/io.tangencylabs.solidsmith.png" alt="" width="32" height="32"> | **Solidsmith** | Parametric 3D CAD by Tangency Labs (community app) | [GitHub](https://github.com/mahmudurrahman420/solidsmith) | |

And [**Smith Suite**](https://github.com/mahmudurrahman420/) itself, our AI image and video studio for artists who want real control.

<br>

<p align="center">
  <a href="https://github.com/mahmudurrahman420/cellsmith/discussions"><img alt="Join the Smith Suite community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Discord is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://github.com/mahmudurrahman420/cellsmith/discussions"><b>https://github.com/mahmudurrahman420/cellsmith/discussions</b></a> ·
  <a href="https://github.com/mahmudurrahman420/">github.com/mahmudurrahman420</a> ·
  <a href="https://github.com/mahmudurrahman420/apps">The Crafting Apps</a> ·
  <a href="https://github.com/mahmudurrahman420/cellsmith">Cellsmith</a>
</p>

## License and credits

Cellsmith is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Shohan (mahmudurrahman420) and the Cellsmith contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

All UI icons are drawn in code and are original; the sample workbooks and every screenshot in
this README were made with Cellsmith itself. Cellsmith bundles no fonts: it uses the fonts
installed on your system.

The Smith Suite name, wordmark and anvil mark in `docs/brand/` are original work by Shohan (mahmudurrahman420), licensed under the project licence (MIT OR Apache-2.0).

<br>

<p align="center">
  <a href="https://github.com/mahmudurrahman420/"><img alt="Smith Suite" src="docs/brand/smithsuite-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://github.com/mahmudurrahman420/">Smith Suite</a> team and community.</sub>
</p>
