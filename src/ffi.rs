use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// Returns the library version string.
///
/// The returned string must be freed with `minecraft_registry_api_free_string`.
///
/// # Safety
///
/// This function is always safe to call.
#[unsafe(no_mangle)]
pub extern "C" fn minecraft_registry_api_version() -> *mut c_char {
    match CString::new(env!("CARGO_PKG_VERSION")) {
        Ok(cs) => cs.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string returned by any `minecraft_registry_api_*` function.
///
/// # Safety
///
/// `s` must be a pointer previously returned by a `minecraft_registry_api_*`
/// function, or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn minecraft_registry_api_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
