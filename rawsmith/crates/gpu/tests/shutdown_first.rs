//! Issue #620: quitting right after the start must not leave the device being created (a long
//! call into the driver, on the `lc-gpu-init` thread) while the process exits. Once
//! `begin_shutdown` is called the device is not created at all.
//! One test in its own process: the shutdown comes before the first use of the GPU.

use std::sync::Arc;
use std::time::Duration;

use rawsmith_develop::DevelopSettings;
use rawsmith_pipeline::{RenderRequest, SourceInfo};

#[test]
fn the_device_is_not_created_once_the_process_is_ending() {
    rawsmith_gpu::begin_shutdown();
    rawsmith_gpu::warm_up();
    assert!(!rawsmith_gpu::available());
    assert_eq!(rawsmith_gpu::adapter_name(), None);
    let src = Arc::new(rawsmith_scenes::demo_library()[0].render(300, 200));
    let rendered = rawsmith_gpu::render(&src, &SourceInfo::default(), &DevelopSettings::default(), &RenderRequest::fit(300, 200), None);
    assert!(rendered.is_none());
    assert!(rawsmith_gpu::wait_idle(Duration::from_secs(60)), "the warm-up thread found the gate closed");
    // `ready()` is "device creation finished" (or the GPU is switched off by the environment)
    let switched_off = rawsmith_gpu::unavailable_reason().is_some_and(|r| r.starts_with("disabled by RAWSMITH_GPU"));
    assert!(switched_off || !rawsmith_gpu::ready(), "no device was created");
}
