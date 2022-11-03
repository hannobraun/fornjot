//! API for checking compatibility between the Fornjot app and a model

use core::slice;

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
pub struct RawVersion {
    /// The pointer to the `str`
    pub ptr: *const u8,

    /// The length of the `str`
    pub len: usize,
}

impl RawVersion {
    /// Convert the `RawVersion` into a string
    ///
    /// # Safety
    ///
    /// Must be a `RawVersion` returned from one of the hidden version functions
    /// in this module.
    #[allow(clippy::inherent_to_string)]
    pub unsafe fn to_string(&self) -> String {
        let slice = slice::from_raw_parts(self.ptr, self.len);
        String::from_utf8_lossy(slice).into_owned()
    }
}

#[no_mangle]
extern "C" fn version_pkg() -> RawVersion {
    RawVersion {
        ptr: VERSION_PKG.as_ptr(),
        len: VERSION_PKG.len(),
    }
}

#[no_mangle]
extern "C" fn version_full() -> RawVersion {
    RawVersion {
        ptr: VERSION_FULL.as_ptr(),
        len: VERSION_FULL.len(),
    }
}
