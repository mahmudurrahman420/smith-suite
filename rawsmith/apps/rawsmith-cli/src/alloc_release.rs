//! Give memory the system allocator keeps after frees back to the system
//! ([`rawsmith_engine::memory::set_release_hook`]). The platform call lives in
//! `rawsmith-sysmem`, the workspace's only crate allowed `unsafe`.

/// Install the hook.
pub fn install() {
    rawsmith_engine::memory::set_release_hook(release);
}

fn release() {
    let _ = rawsmith_sysmem::release_free_memory();
}
