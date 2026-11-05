# Compatibility with the SolveCraft name

Solidsmith was first called SolveCraft, and builds up to 0.4 kept that name in their
identifiers after the displayed name changed. 0.5.0 renamed the binaries, release assets and
packages; the release after it renames everything else: crates `solidsmith-*`, binaries `solidsmith` and `solidsmith-cli`, `.solidsmith`
designs (saved as `.sc` since; see section 1), the `"format": "solidsmith/N"` tag, `SOLIDSMITH_*` environment variables, settings
and recovery folders, the MCP server name, docs, CI and packaging.

The old name survives **only** in the shims below (plus their tests, this file, and changelog /
release notes). `git grep -i solvecraft` must list nothing else. When you add or remove a
shim, update this file in the same PR.

## 1. `.solvecraft` and `.solidsmith` design files

New designs are saved as `.sc`. Files with either older extension still open everywhere: `.solvecraft`
(saved before the rename) and `.solidsmith` (saved before designs moved to `.sc`). One constant
names the extension new designs get, `DESIGN_EXT` (`sc`), and one list names every extension a
design may have, `DESIGN_EXTS` (`sc`, then `LEGACY_DESIGN_EXTS`: `solidsmith`, `solvecraft`);
code uses those, never its own string.

| Path in | Where |
|---|---|
| Extension → design format | `crates/io/src/lib.rs` (`DESIGN_EXTS`, `is_design_ext`, `is_design_path`, format detection) |
| Open dialog filters (desktop and browser) | `crates/io/src/formats.rs` (the design row lists every entry of `DESIGN_EXTS`), shown by `apps/solidsmith/src/main.rs` (`services`) |
| Drag-drop, recent files, `doc.open`, MCP `open`, CLI `eval`/`run`, Insert/Link/Relink design | go through the same format detection in `solidsmith_io` (`is_design_container_path` for links) |
| Browser file picker | `apps/solidsmith-web/src/disk.rs` (`open_options`, `opens_design`) |
| Windows | `assets/platforms/windows/solidsmith.wxs`: `.sc` is the design ProgId's extension; `.solidsmith` and `.solvecraft` point at the same ProgId and are in `SupportedTypes` |
| macOS | `assets/platforms/macos/Info.plist.in`: `sc`, `solidsmith` and `solvecraft` in the design document type and its `io.tangencylabs.solidsmith.design` UTI |
| Linux / FreeBSD / Flatpak / Snap | `assets/platforms/linux/io.tangencylabs.solidsmith.mime.xml`: `*.sc`, `*.solidsmith` and `*.solvecraft` globs under `application/x-solidsmith` (plus the `application/x-solvecraft` alias); the `.desktop` file names that type |

Opening a `.solidsmith` file and pressing **Save** writes that same file: nothing is renamed behind
the user's back. **Save As** (and every new design, autosave, crash report and component
extracted to its own design) offers `.sc`, and leaves the old file as it was.

Recent-files entries that point at an older file keep working: they are plain paths and
open through the same detection. Autosaves and crash reports that earlier releases wrote
(`<id>.solidsmith`, `<id>.solvecraft`, `design.solidsmith`) are still listed and recovered
(`solidsmith_engine::recovery`, `solidsmith_engine::crash::read`), and the browser app still
recovers `recovery.solidsmith` / `recovery.solvecraft` from its storage. The browser app also
lists designs it stored under the old extensions, and keyboard-shortcut files exported as
`solvecraft-shortcuts/1` still import (`crates/ui-egui/src/toolbar_custom.rs`). The designs in
`apps/solidsmith-cli/tests/corpus` keep their `.solidsmith` names on purpose: they are files saved
by earlier releases. Tests: `crates/io` (`Format::from_name`, `is_design_path`, the import filter
groups), `legacy_extension_and_format_tag_still_open` (CLI, `run_results.rs`),
`legacy_design_files_and_resource_uris` (MCP `open` and `save`),
`saving_an_opened_legacy_design_keeps_its_name` (engine),
`autosave_from_a_legacy_folder_is_listed_and_loads`, `autosaves_are_written_as_sc` and
`a_report_from_an_earlier_release_still_finds_its_design`.

## 2. The `solvecraft/N` format tag

Designs record their format as `"format": "solidsmith/N"`. Files saved by builds up to 0.5 say
`"format": "solvecraft/N"`; `crates/doc/src/format.rs` (`version_of`) reads both prefixes and
upgrades them as usual (the file is re-tagged `solidsmith/N` when saved).

Not backward-compatible: builds up to 0.5 don't know the new tag and refuse files saved by
later builds. Test: `version_of` accepts both prefixes; legacy-tagged designs load and are
re-tagged on save.

## 3. `SOLVECRAFT_*` environment variables

Every runtime variable is read through one helper, `solidsmith_geom::env` (`var`, `var_os`):
it reads `SOLIDSMITH_<NAME>` and falls back to `SOLVECRAFT_<NAME>` when the new one is unset.
The shell entry points do the same: `packaging/linux/AppRun`
(`SOLIDSMITH_NO_DESKTOP_INTEGRATION`) and `packaging/env.sh` (`SOLIDSMITH_VERSION`,
`SOLIDSMITH_BUILD_DATE`). Tests: the helper's unit tests (new wins, old is the fallback,
neither → unset).

## 4. Settings, preferences, autosave and recovery folders

New folders use the Solidsmith name:

| | Linux | macOS | Windows |
|---|---|---|---|
| Preferences, graphics settings | `~/.local/share/solidsmith` | `~/Library/Application Support/Solidsmith` | `%APPDATA%\Solidsmith\data` |
| Recovery (autosave) | `…/solidsmith/recovery` | `…/Solidsmith/recovery` | `%LOCALAPPDATA%\Solidsmith\recovery` |

On first launch, if the new folder doesn't exist but the old one (`solvecraft` / `SolveCraft`)
does, it is **copied** to the new location; the old folder is never deleted or changed. The
window and UI state that eframe kept under `io.github.mahmudurrahman420.solvecraft` is copied the same way.
Autosaves copied from the old folder (`<id>.solvecraft`) are listed and recovered like new ones.
The preferences key in eframe's storage is `solidsmith.prefs`; `solvecraft.prefs` is read when
it is missing (desktop and browser, `solidsmith_ui_egui::prefs::stored`). The browser app also
reads files and the autosave it kept under the old names in local storage and OPFS
(`apps/solidsmith-web/src/files.rs`). A 3DxWare profile the driver made for the old executable
name is read until it makes a new one (`apps/solidsmith/src/space_mouse.rs`).
In the Flatpak sandbox these folders live under `~/.var/app/<app id>/data`; for data kept under
the app id of releases before 0.8, see [Linux app ID](#linux-app-id).
The thumbnail cache is not migrated; it is rebuilt. The copy helper is
`solidsmith_io::adopt_legacy_dir`, used by the desktop app (`adopt_legacy_settings`) and
`solidsmith_engine::recovery::default_dir`. Tests: `legacy_dir` (copy once, never overwrite,
old folder intact), `first_launch_adopts_the_previous_release_settings`,
`autosave_from_a_legacy_folder_is_listed_and_loads`, `recovery_dirs_and_their_legacy_names`,
`stored_prefs_fall_back_to_the_legacy_key`.

## 5. `solvecraft-cli` alias

A tiny `solvecraft-cli` binary (`apps/solidsmith-cli/src/alias.rs`) ships in every package for
at least one release (since 0.5.0). It prints a one-line deprecation note to stderr and runs the
`solidsmith-cli` next to it with the same arguments, stdin/stdout and exit code, so scripts
and MCP client configs that call `solvecraft-cli` keep working. Test: `apps/solidsmith-cli/tests/legacy_alias.rs` (forwards `--version` and the exit status;
the note goes to stderr only, so stdout stays clean for MCP).

There is no alias for the desktop binary: launchers, file associations and Start-menu entries
installed by the packages point at `solidsmith`.

## 6. MCP

`serverInfo.name` is now `solidsmith`. The name a client shows is chosen by the user when the
server is added, so an existing setup such as

```sh
claude mcp add solvecraft -- solvecraft-cli mcp
```

keeps working (through the alias above). New setups use:

```sh
claude mcp add solidsmith -- solidsmith-cli mcp
```

Switch before the alias is removed. Resources are listed as `solidsmith://design` and
`solidsmith://commands`; the old `solvecraft://` URIs still read (`crates/mcp/src/server.rs`,
test `legacy_design_files_and_resource_uris`).

## Package IDs

| Package | Decision | Why |
|---|---|---|
| Flatpak app ID / Linux desktop ID | changed in 0.8 from `io.github.mahmudurrahman420.solvecraft` to `io.github.mahmudurrahman420.solidsmith`, then with the move to Tangency Labs to `io.tangencylabs.solidsmith` | The project's own reverse-DNS domain. The `.desktop`, metainfo and MIME file names, the icon name, and the Wayland `app_id` / X11 class follow it. Existing installs are moved over; see [Linux app ID](#linux-app-id). |
| Flatpak remote file | `solidsmith.flatpakrepo`, plus a legacy `solvecraft.flatpakrepo` copy | Old links keep working; existing remotes point at the repo URL, which is unchanged. |
| macOS bundle ID | changed to `io.github.mahmudurrahman420.solidsmith`, then to `io.tangencylabs.solidsmith`; the CLI's signing identifier is `io.tangencylabs.solidsmith.cli` (was `io.github.mahmudurrahman420.solidsmith-cli`) | No updater depends on it; the app is drag-installed and replaces the old `Solidsmith.app`. Settings live in folders named after the product, not the bundle ID, so they stay. macOS ties privacy permissions (such as access to Documents or Downloads) to the bundle ID, so it asks for them once more after the update. |
| MSI `UpgradeCode` | **kept** (also through the `io.tangencylabs` move; the Windows build sets no AppUserModelID and its ProgIDs `Solidsmith.Design` / `Solidsmith.Model` don't carry the app ID, so nothing there changes) | So the new MSI upgrades the old install in place. Product name is Solidsmith; the default folder becomes `Program Files\Solidsmith` (the major upgrade removes the old folder). A custom install folder isn't remembered across this one upgrade. |
| deb / rpm package name | renamed to `solidsmith`; deb `Replaces`/`Conflicts`/`Provides: solvecraft`, rpm `Obsoletes`/`Provides: solvecraft` | Installing the new package replaces the old one instead of installing side by side. |
| AppImage | `solidsmith-<version>-linux-<arch>.AppImage`; the release also carries a copy of its `.zsync` under the old `solvecraft-…` name | AppImages already installed look for the old-named `.zsync`, so AppImageUpdate keeps finding updates. |

## Linux app ID

Releases up to 0.7 used `io.github.mahmudurrahman420.solvecraft`; 0.8 until the move to Tangency Labs used
`io.github.mahmudurrahman420.solidsmith`; releases since use `io.tangencylabs.solidsmith` everywhere (Flatpak,
deb, rpm, AppImage, Snap, FreeBSD, Nix, and the macOS bundle). The settings and autosave folders
(`solidsmith` / `Solidsmith`) are named after the product, not the app ID, so outside the Flatpak
sandbox they don't move. The shims cover both earlier IDs:

| Shim | Where | What it does |
|---|---|---|
| End-of-life rebase in the Flatpak repository | `packaging/flatpak-site.sh` | Each release's Pages deploy keeps the old `app/io.github.mahmudurrahman420.solidsmith/<arch>/stable` and `app/io.github.mahmudurrahman420.solvecraft/<arch>/stable` refs (mirrored from the previous deploy) and marks each with `flatpak build-commit-from --end-of-life-rebase=<old id>=io.tangencylabs.solidsmith` (once the repository has the new ref). A ref already marked with a rebase to an older id (`io.github.mahmudurrahman420.solvecraft` pointed at `io.github.mahmudurrahman420.solidsmith`) is marked again, so every old install moves in one step. `flatpak update` then says the old app is end-of-life and offers to replace it with the new one; accepting installs `io.tangencylabs.solidsmith`, uninstalls the old app, and Flatpak moves `~/.var/app/<old id>` to the new id's folder (leaving a symlink at the old path) on the first launch, so settings and autosaves come along unchanged. |
| AppStream `<replaces>` / `<provides>` | `assets/platforms/linux/io.tangencylabs.solidsmith.metainfo.xml.in` | Software centres treat the new id as the successor of `io.github.mahmudurrahman420.solidsmith` and `io.github.mahmudurrahman420.solvecraft` and of their `.desktop` ids. |
| `X-Flatpak-RenamedFrom` | `assets/platforms/linux/io.tangencylabs.solidsmith.desktop` (both old `.desktop` ids) | Desktops that honour it (GNOME Shell) keep favourites and pinned launchers pointing at the renamed entry. |
| Read-only access to the old Flatpak data folders | `--filesystem=~/.var/app/io.github.mahmudurrahman420.solidsmith:ro` and `--filesystem=~/.var/app/io.github.mahmudurrahman420.solvecraft:ro` in `packaging/linux/flatpak/io.tangencylabs.solidsmith.bundle.yml`; `solidsmith_io::legacy_flatpak_paths` (`PREVIOUS_APP_IDS`, newest first), used by `adopt_legacy_settings` and `recovery::default_dir` | When the new app was installed next to an old one (for example from a `.flatpak` bundle, which has no update feed) instead of through the rebase, its first launch copies settings and autosaves from the newest old app's folder that exists, as in section 4. The old folders are not changed. |
| The old window `app_id` as eframe's legacy storage key | `adopt_legacy_settings` (`LEGACY_APP_ID`) | Window state that releases before the settings rename kept under `~/.local/share/io.github.mahmudurrahman420.solvecraft` is still copied (section 4). |
| Old AppImage menu entries | `packaging/linux/AppRun` | An AppImage's first run removes the `io.github.mahmudurrahman420.solidsmith.desktop` and `io.github.mahmudurrahman420.solvecraft.desktop` entries and icons that an earlier AppImage installed in `~/.local/share` (only entries it wrote itself), so the menu doesn't show the app twice. Test: `packaging/tests/apprun_test.py`. |

deb and rpm need no shim: the package (`solidsmith`) is unchanged, and an upgrade removes the old
`/usr/share/applications/io.github.mahmudurrahman420.*.desktop`, metainfo, MIME and icon files
because the new version doesn't ship them; the post-install script refreshes the desktop, MIME
and icon caches, so no stale menu entry is left. The Snap is unaffected apart from its
`common-id`: snapd names its menu entry after the snap.

Flatpak installed from a downloaded `.flatpak` bundle (no update feed): install the new bundle,
launch it once (settings and autosaves are copied), then
`flatpak uninstall --user io.github.mahmudurrahman420.solidsmith` (or `io.github.mahmudurrahman420.solvecraft`).

**Removal:** keep the end-of-life refs in the Flatpak repository for at least six months after
the release that moved to `io.tangencylabs.solidsmith` and at least two minor releases; removing
them strands any install that hasn't run `flatpak update` since. The read-only filesystem grants
and `legacy_flatpak_paths` can go at the same time. `<replaces>`, `<provides>` and `X-Flatpak-RenamedFrom` are harmless and can stay.
Tests: `flatpak_twins_map_our_sandbox_data_to_the_old_app_ids`,
`first_launch_adopts_the_previous_release_settings`.

## macOS

Releases before 0.4 were installed as `SolveCraft.app`. To update, quit the app, drag
`Solidsmith.app` from the DMG onto Applications, move `SolveCraft.app` to the Trash, and
replace any Dock shortcut with the old name (open `Solidsmith.app`, then
**Options > Keep in Dock**). Designs and settings are kept (see 4).

## Flatpak remotes added from the former website address

The feed has moved twice: from the SolveCraft address, then from `bherbruck.github.io/solidsmith`
when the project moved to `docs.solidsmith.io` (#1920). A remote added from either address no
longer finds updates. Point it at the current one:
`flatpak remote-modify --user --url=https://docs.solidsmith.io/flatpak/repo/ solidsmith`
(use your remote's name: `solvecraft` for one added before 0.4), then `flatpak update --user`.

## Draft design containers

The #303 draft used `solvecraft-container/1`. `crates/io/src/design.rs` accepts that
marker alongside `solidsmith-container/1`, and writes only the new marker. Optional
old previews are discarded; required document/blob validation is unchanged. The
`draft_container_marker_remains_readable` regression covers this shim.

## Split pieces in designs saved before format 14

Builds before format 14 named the pieces a cut splits a body into in the old kernel's shell
order, and the design's later features name the piece they work on by that name (`Body1`,
`Body1 (1)`). SMLib orders pieces otherwise. Such a feature (`shell_piece_order` in the file)
names its pieces so its later features find them: each name goes to the piece the points those
features picked lie on. A name no later feature uses keeps SMLib's order. Designs saved since
format 14 name pieces after their faces and are not affected.
