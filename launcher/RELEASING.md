# Releasing Smith Suite apps and the launcher

Tags on `mahmudurrahman420/smith-suite` are the release source. The launcher never talks to git
directly. CI builds the Windows asset, then regenerates `catalog.json` onto the rolling `catalog`
release. The backend serves that file from `GET /catalog`.

## Tag names

| Tag | What CI builds | Asset the catalog prefers |
|---|---|---|
| `pixelsmith-v1.2.0` (any of the 16 app ids) | that app | `<app>-<version>-win-x64.zip` containing `<app>.exe`, else a `.exe` |
| `framesmith-v0.10.0` | Frame Smith. The crate is `filmcraft` and the built file is `filmcraft.exe`; CI renames it to `framesmith.exe` | `framesmith-<version>-win-x64.zip` containing `framesmith.exe` |
| `launcher-v1.0.1` | `SmithCreativeCloud.exe` and, when NSIS is available, `SmithCreativeCloud-Setup.exe` | the file named exactly `SmithCreativeCloud.exe` |
| `catalog` | nothing; this tag is the rolling catalog | `catalog.json` |

The version is exactly `MAJOR.MINOR.PATCH`. Drafts, prereleases, and tags like
`pixelsmith-v1.2.0-rc1` are ignored. `catalog` does not match the build pattern.

## Cut an app release

```sh
git tag pixelsmith-v1.2.0
git push origin pixelsmith-v1.2.0
```

`Build Windows app` runs on the tag, builds `pixelsmith.exe`, zips it as
`pixelsmith-1.2.0-win-x64.zip` with `pixelsmith.exe` at the root of the archive, and uploads both
to the GitHub Release for that tag. When that workflow finishes, `Catalog` rebuilds `catalog.json`
and uploads it to the `catalog` release with `--clobber`.

A release published from the GitHub UI, with the asset already attached, also triggers `Catalog`.

## Cut a Frame Smith release

Frame Smith is the video editor in `framesmith/`. Its desktop binary crate is `filmcraft`
(`apps/filmcraft`), which produces `filmcraft.exe`. The catalog id is `framesmith` and the
published asset is named Frame Smith.

```sh
git tag framesmith-v0.10.0
git push origin framesmith-v0.10.0
```

`Build Windows app` matches `framesmith-vX.Y.Z` and, on `windows-latest`, runs:

```sh
export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS="-C target-feature=+crt-static"
export FILMCRAFT_REQUIRE_WINRES=1
cargo build --release --locked -p filmcraft --target x86_64-pc-windows-msvc
```

Rust stable 1.95 or later (edition 2024) and the MSVC Build Tools plus the Windows SDK (`rc.exe`,
for `assets/app-icon/filmcraft.ico`) are already on `windows-latest`. CI copies `filmcraft.exe` to
`framesmith.exe` and uploads that file plus `framesmith-<version>-win-x64.zip` with
`framesmith.exe` at the root of the archive. The catalog default entry is `framesmith.exe`, so the
launcher opens it the same way as the other apps.

The same tag also builds the Android APK (`Android APK` workflow) and attaches
`Framesmith-<version>.apk` to that release. Signing uses the `FRAMESMITH_KEYSTORE_*` secrets when
they are set; otherwise the APK is signed with a debug key.

## Cut a launcher release

```sh
# version in launcher/Cargo.toml is the version the binary reports
git tag launcher-v1.0.1
git push origin launcher-v1.0.1
```

The self-update downloads `SmithCreativeCloud.exe` only. The NSIS setup exe is for a first install
into `%LOCALAPPDATA%\SmithSuite\launcher` and is not the catalog entry. Do not rename the portable
exe; the catalog picks `SmithCreativeCloud.exe` and skips names containing `setup`.

## Release body lines

The catalog workflow reads these lines out of the GitHub release body and removes them from the
notes people see:

```text
entry: pixelsmith.exe
min_launcher: 1.0.0
mandatory: true
```

`entry` defaults to `<app>.exe` for a zip and to the asset file name for an exe. It must be a
relative path with no `..`. `min_launcher` defaults to `1.0.0`. `mandatory: true` applies to
launcher releases and blocks app installs until the launcher updates. Anything else in the body is
the release notes.

## sha256 and size

GitHub's asset `digest` (`sha256:<hex>`) and `size` are copied into the catalog. If a digest is
missing, the catalog job downloads the asset to hash it, which is slow for the large apps. Prefer
keeping the digest GitHub already stores.

## Manual runs

Actions → **Build Windows app** → choose one app, `launcher`, or `all`. That uploads artifacts and
does not publish a release.

Actions → **Catalog** rebuilds `catalog.json` from whatever releases already exist.

Actions → **Launcher CI** runs on pull requests that touch `launcher/` and builds
`x86_64-pc-windows-msvc` plus the Linux test suite.

## Do not delete the `catalog` release

Every launcher polls it (through the backend). Deleting the tag makes `GET /catalog` return
503 `catalog_unavailable` until the workflow publishes the file again.
