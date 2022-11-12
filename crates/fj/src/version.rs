//! API for checking compatibility between the Fornjot app and a model

use std::{ffi::CStr, os::raw::c_char};

/// The Fornjot package version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
///
/// This is just the version specified in the Cargo package, which will stay
/// constant between releases, even though changes are made throughout. A match
/// of this version does not conclusively determine that the app and a model are
/// compatible.
pub static VERSION_PKG: &str = env!("FJ_VERSION_PKG");

/// The full Fornjot version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
pub static VERSION_FULL: &str = env!("FJ_VERSION_FULL");

/// C-ABI-compatible representation of a version
///
/// Used by the Fornjot application to check for compatibility between a model
/// and the app.
#[repr(C)]
pub struct RawVersion(*const c_char);

impl RawVersion {
    /// Convert the `RawVersion` into a string
    ///
    /// # Safety
    ///
    /// Must be a `RawVersion` returned from one of the hidden version functions
    /// in this module.
    #[allow(clippy::inherent_to_string)]
    pub unsafe fn to_string(&self) -> String {
        CStr::from_ptr(self.0)
            .to_str()
            .expect("Failed to convert RawVersion into String")
            .to_owned()
    }
}

#[no_mangle]
extern "C" fn version_pkg() -> RawVersion {
    static C_VERSION_PKG: &str = concat!(env!("FJ_VERSION_PKG"), "\0");
    RawVersion(C_VERSION_PKG.as_ptr() as *const c_char)
}

#[no_mangle]
extern "C" fn version_full() -> RawVersion {
    static C_VERSION_FULL: &str = concat!(env!("FJ_VERSION_FULL"), "\0");
    RawVersion(C_VERSION_FULL.as_ptr() as *const c_char)
}
