# Framesmith Android packaging

A Gradle project with no Java or Kotlin: `android.app.NativeActivity` loads `libframesmith.so`
(built from `apps/framesmith-android` by `cargo ndk` into `app/src/main/jniLibs/<abi>/`).
See the main README for build steps; CI does it in [`../../.github/workflows/android.yml`](../../.github/workflows/android.yml).

- App id: `com.framesmith.editor` (change `applicationId` in `app/build.gradle.kts` before
  publishing if you prefer another).
- Icons: `app/src/main/res/mipmap-*/ic_launcher.png`; `framesmith-icon-512.png` is the store icon.
