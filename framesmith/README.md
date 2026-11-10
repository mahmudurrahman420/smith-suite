# Framesmith

**A full non-linear video editor for your phone — portrait and landscape — built on the FilmCraft engine.**

Framesmith puts a professional editor in your pocket: multi-track timeline, every trim, colour
grading with scopes, 90+ effects, keyframes, a real audio mixer with broadcast loudness meters,
titles, captions and H.264/HEVC/ProRes export — all in pure Rust, no FFmpeg, no account, no
subscription, no cloud. Your media never leaves the phone.

It is a fork of [FilmCraft](https://github.com/storytold/filmcraft) by the ArtCraft team and
contributors (MIT OR Apache-2.0). FilmCraft's engine and panels are unchanged underneath; Framesmith
adds a touch layout and an Android app. Not affiliated with or endorsed by the ArtCraft team.

This folder is Frame Smith in Smith Suite, taken from commit `fd022fa` with no git history.
Tag `framesmith-vX.Y.Z` (the workspace version here is `0.10.0`) to publish the Windows desktop
build and the Android APK. See [Releasing](../launcher/RELEASING.md). The desktop crate is still
`filmcraft`; the Windows asset is renamed to `framesmith.exe`.

The large FilmCraft screenshots that lived in `docs/images/*.png` are not in this copy.
[ATTRIBUTION.md](ATTRIBUTION.md) and the `.attribution` files still record them.

## The phone layout

| Portrait | Landscape |
|---|---|
| Top bar (menu, project, undo/redo, **Export**), preview, transport, timeline, and a toolbar along the bottom | A tool rail on the left, preview and timeline in the middle, and sheets (colour, effects…) on the right beside the preview |

- **The toolbar follows the selection.** Nothing selected: Media, Audio, Text, Effects,
  Transitions, Color, Canvas (9:16, 16:9, 1:1, 4:5…), Captions, Marker, Mixer, Panels. Clips
  selected: Split, Delete, Duplicate, Speed, Volume, Adjust (keyframes), Color, Effects, Transform,
  Freeze, Extract audio, Fill frame, Copy, Disable, Nest.
- **Every desktop panel is one tap away** in a sheet (Panels), and the whole menu bar is behind ☰.
- **Touch gestures:** tap = click, drag = drag/trim/move, long-press = context menu, two fingers =
  scroll the timeline and pinch to zoom it. The back button closes whatever is on top.
- **Add from phone:** an in-app picker for the camera roll, videos, pictures, downloads and music;
  picked clips are inserted at the playhead (the first ones start a sequence shaped like them).
- **Export** in one tap (Quick Export presets) or with every setting; finished files land in
  `Movies/` and show in the gallery.
- Auto-save and crash recovery as on the desktop; projects live in `Documents/Framesmith`.

## Get the APK

The [Android APK workflow](../.github/workflows/android.yml) builds it on GitHub when `framesmith/`
changes: open **Actions ▸ Android APK ▸** the latest run **▸ Artifacts ▸ Framesmith-apk**, unzip, and
install the `.apk` on your phone (allow installing from your browser/files app when asked). Push a
tag like `framesmith-v0.10.0` to also publish it as a GitHub release.

To sign with your own key (needed for Google Play, and so updates install over each other), add
these repository secrets: `FRAMESMITH_KEYSTORE_BASE64` (your `.jks`, base64), `FRAMESMITH_KEYSTORE_PASSWORD`,
`FRAMESMITH_KEY_ALIAS`, `FRAMESMITH_KEY_PASSWORD`. Without them the APK is signed with a debug key.

Requires Android 8.0+ (API 26), 64-bit ARM.

### Build it yourself

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
export ANDROID_NDK_HOME=/path/to/android-ndk      # Android Studio ▸ SDK Manager ▸ NDK
cargo ndk --platform 26 -t arm64-v8a -o android/app/src/main/jniLibs build --release -p framesmith-android
cd android && gradle assembleRelease               # → app/build/outputs/apk/release/app-release.apk
```

### Try the phone layout on a computer

```sh
FRAMESMITH_MOBILE=1 FRAMESMITH_WINDOW=390x844 cargo run --release -p filmcraft -- --demo   # portrait
FRAMESMITH_MOBILE=1 FRAMESMITH_WINDOW=844x390 cargo run --release -p filmcraft -- --demo   # landscape
```

Without `FRAMESMITH_MOBILE` the same binary is the full desktop editor.

## Where things are

| | |
|---|---|
| `crates/ui-egui/src/mobile/` | The phone layout: shell (`mod.rs`), touch gestures (`touch.rs`), file picker (`picker.rs`) |
| `apps/framesmith-android/` | The Android app (NativeActivity entry, JNI for permissions, insets, media scanner) |
| `android/` | Gradle project that packages `libframesmith.so` into the APK |
| everything else | FilmCraft: engine, codecs, effects, export — see [docs/FILMCRAFT-README.md](docs/FILMCRAFT-README.md) and [docs/architecture.md](docs/architecture.md) |

## Status

Young. The editing engine is FilmCraft's (alpha). On phones, decoding runs on the CPU with
Framesmith's own decoders (no MediaCodec hardware decoding yet), so 4K and long clips are slower
than on a computer; lower Settings ▸ Playback resolution if playback stutters.

## License

MIT OR Apache-2.0, at your option, as FilmCraft. See [NOTICE](NOTICE), [LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE) and [ATTRIBUTION.md](ATTRIBUTION.md).
