//! The few Android framework calls Framesmith needs, through JNI: runtime permissions, the shared
//! storage path, system bar insets, the media scanner, opening links and going to the background.
//!
//! The only module of this crate allowed `unsafe` (turning the activity's raw `JavaVM` and
//! `Activity` pointers into JNI handles); every call returns a `Result` internally and failures are
//! logged, never panicked on: none of these is essential to editing.

#![allow(unsafe_code)]

use std::path::PathBuf;

use jni::JavaVM;
use jni::objects::{JObject, JValue};
use winit::platform::android::activity::AndroidApp;

type Res<T> = Result<T, jni::errors::Error>;

/// Run `f` with a JNI environment attached to this thread and the activity object.
fn with_activity<T>(app: &AndroidApp, what: &str, f: impl FnOnce(&mut jni::JNIEnv, &JObject) -> Res<T>) -> Option<T> {
    // SAFETY: `vm_as_ptr` is the process's `JavaVM*`, valid for the life of the process.
    let vm = match unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) } {
        Ok(vm) => vm,
        Err(e) => {
            log::warn!("{what}: no Java VM: {e}");
            return None;
        }
    };
    let mut env = match vm.attach_current_thread() {
        Ok(env) => env,
        Err(e) => {
            log::warn!("{what}: can't attach to the Java VM: {e}");
            return None;
        }
    };
    // SAFETY: `activity_as_ptr` is a global reference to the `NativeActivity`, valid while the
    // activity exists (all the time Framesmith runs); `JObject` borrows it without freeing it.
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
    let r = f(&mut env, &activity);
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
    }
    match r {
        Ok(v) => Some(v),
        Err(e) => {
            log::warn!("{what}: {e}");
            None
        }
    }
}

fn sdk_int(env: &mut jni::JNIEnv) -> Res<i32> {
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()
}

/// Ask for the media and microphone permissions not granted yet (the system shows its dialog).
pub fn request_permissions(app: &AndroidApp) {
    with_activity(app, "permissions", |env, activity| {
        let sdk = sdk_int(env)?;
        if sdk < 23 {
            return Ok(()); // granted at install time
        }
        let mut missing = Vec::new();
        for p in crate::permissions_for(sdk) {
            let name = env.new_string(p)?;
            let granted = env.call_method(activity, "checkSelfPermission", "(Ljava/lang/String;)I", &[JValue::Object(&name)])?.i()?;
            if granted != 0 {
                missing.push(p);
            }
        }
        if missing.is_empty() {
            return Ok(());
        }
        let string = env.find_class("java/lang/String")?;
        let array = env.new_object_array(missing.len() as i32, string, JObject::null())?;
        for (i, p) in missing.iter().enumerate() {
            let s = env.new_string(p)?;
            env.set_object_array_element(&array, i as i32, s)?;
        }
        env.call_method(activity, "requestPermissions", "([Ljava/lang/String;I)V", &[JValue::Object(&array), JValue::Int(1)])?;
        log::info!("asked for {missing:?}");
        Ok(())
    });
}

/// The shared storage root (`Environment.getExternalStorageDirectory()`).
pub fn shared_storage(app: &AndroidApp) -> Option<PathBuf> {
    with_activity(app, "shared storage", |env, _| {
        let dir = env.call_static_method("android/os/Environment", "getExternalStorageDirectory", "()Ljava/io/File;", &[])?.l()?;
        if dir.is_null() {
            return Ok(None);
        }
        let path = env.call_method(&dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?.l()?;
        let path = jni::objects::JString::from(path);
        let path: String = env.get_string(&path)?.into();
        Ok(Some(PathBuf::from(path)))
    })
    .flatten()
}

/// The system bars' and cutout's insets in pixels: top, bottom, left, right. None before the
/// window has been laid out.
pub fn system_insets(app: &AndroidApp) -> Option<[i32; 4]> {
    with_activity(app, "insets", |env, activity| {
        let window = env.call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?.l()?;
        if window.is_null() {
            return Ok(None);
        }
        let decor = env.call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?.l()?;
        if decor.is_null() {
            return Ok(None);
        }
        let insets = env.call_method(&decor, "getRootWindowInsets", "()Landroid/view/WindowInsets;", &[])?.l()?;
        if insets.is_null() {
            return Ok(None);
        }
        let mut get = |m: &str| env.call_method(&insets, m, "()I", &[]).and_then(|v| v.i());
        Ok(Some([get("getSystemWindowInsetTop")?, get("getSystemWindowInsetBottom")?, get("getSystemWindowInsetLeft")?, get("getSystemWindowInsetRight")?]))
    })
    .flatten()
}

/// Tell the media scanner about a written file, so it shows in the gallery and other apps.
pub fn scan_media(app: &AndroidApp, path: &str) {
    with_activity(app, "media scan", |env, activity| {
        let string = env.find_class("java/lang/String")?;
        let paths = env.new_object_array(1, string, JObject::null())?;
        let p = env.new_string(path)?;
        env.set_object_array_element(&paths, 0, p)?;
        env.call_static_method(
            "android/media/MediaScannerConnection",
            "scanFile",
            "(Landroid/content/Context;[Ljava/lang/String;[Ljava/lang/String;Landroid/media/MediaScannerConnection$OnScanCompletedListener;)V",
            &[JValue::Object(activity), JValue::Object(&paths), JValue::Object(&JObject::null()), JValue::Object(&JObject::null())],
        )?;
        Ok(())
    });
}

/// Open a web address in the browser.
pub fn open_url(app: &AndroidApp, url: &str) -> Result<(), String> {
    with_activity(app, "open link", |env, activity| {
        let s = env.new_string(url)?;
        let uri = env.call_static_method("android/net/Uri", "parse", "(Ljava/lang/String;)Landroid/net/Uri;", &[JValue::Object(&s)])?.l()?;
        let action = env.new_string("android.intent.action.VIEW")?;
        let intent = env.new_object("android/content/Intent", "(Ljava/lang/String;Landroid/net/Uri;)V", &[JValue::Object(&action), JValue::Object(&uri)])?;
        env.call_method(activity, "startActivity", "(Landroid/content/Intent;)V", &[JValue::Object(&intent)])?;
        Ok(())
    })
    .ok_or_else(|| "no app can open this link".to_string())
}

/// Leave for the home screen (the back button with nothing left to close); the project stays open.
pub fn move_task_to_back(app: &AndroidApp) {
    with_activity(app, "back", |env, activity| {
        env.call_method(activity, "moveTaskToBack", "(Z)Z", &[JValue::Bool(1)])?;
        Ok(())
    });
}
