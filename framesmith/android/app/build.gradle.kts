// Framesmith's APK: no Java or Kotlin code. android.app.NativeActivity loads libframesmith.so,
// which `cargo ndk` builds from apps/framesmith-android into src/main/jniLibs/<abi>/ before this
// runs (see ../../.github/workflows/android.yml and android/README.md).
plugins {
    id("com.android.application")
}

// Release signing from the environment (CI secrets); without them the release APK is signed with
// the debug key, which installs fine for testing but can't be published to Google Play.
val keystorePath: String? = System.getenv("FRAMESMITH_KEYSTORE")
val versionNameEnv: String = System.getenv("FRAMESMITH_VERSION_NAME") ?: "0.1.0"
val versionCodeEnv: Int = System.getenv("FRAMESMITH_VERSION_CODE")?.toIntOrNull() ?: 1

android {
    namespace = "com.framesmith.editor"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.framesmith.editor"
        minSdk = 26
        targetSdk = 35
        versionCode = versionCodeEnv
        versionName = versionNameEnv
    }

    signingConfigs {
        if (keystorePath != null) {
            create("release") {
                storeFile = file(keystorePath)
                storePassword = System.getenv("FRAMESMITH_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("FRAMESMITH_KEY_ALIAS")
                keyPassword = System.getenv("FRAMESMITH_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            signingConfig = if (keystorePath != null) signingConfigs.getByName("release") else signingConfigs.getByName("debug")
        }
    }

    packaging {
        // the Rust library is stripped by cargo; keep it as built
        jniLibs.keepDebugSymbols += "**/libframesmith.so"
        jniLibs.useLegacyPackaging = true
    }
}
