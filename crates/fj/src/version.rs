//! API for checking compatibility between the Fornjot app and a model

use std::{fmt, slice};

/// The Fornjot package version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
///
/// This is just the version specified in the Cargo package, which will stay
/// constant between releases, even though changes are made throughout. A match
/// of this version does not conclusively determine that the app and a model are
/// compatible.
#[no_mangle]
pub static VERSION_PKG: Version =
    Version::from_static_str(env!("FJ_VERSION_PKG"));

/// The full Fornjot version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
#[no_mangle]
pub static VERSION_FULL: Version =
    Version::from_static_str(env!("FJ_VERSION_FULL"));

/// C-ABI-compatible representation of a version
///
/// Used by the Fornjot application to check for compatibility between a model
/// and the app.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Version {
    ptr: *const u8,
    len: usize,
}

impl Version {
    const fn from_static_str(s: &'static str) -> Self {
        Self {
            ptr: s.as_ptr(),
            len: s.len(),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is sound. We only ever create `ptr` and `len` from static
        // strings.
        let slice = unsafe { slice::from_raw_parts(self.ptr, self.len) };

        write!(f, "{}", String::from_utf8_lossy(slice).into_owned())
    }
}

// The only reason this is not derived automatically, is that `Version` contains
// a `*const u8`. `Version` can still safely be `Send`, for the following
// reasons:
// - The field is private, and no code in this module uses it for any write
//   access, un-synchronized or not.
// - `Version` can only be constructed from strings with a static lifetime, so
//   it's guaranteed that the pointer is valid over the whole lifetime of the
//   program.
unsafe impl Send for Version {}

// There is no reason why a `&Version` wouldn't be `Send`, so per definition,
// `Version` can be `Sync`.
unsafe impl Sync for Version {}

#[no_mangle]
extern "C" fn version_pkg() -> Version {
    VERSION_PKG
}

#[no_mangle]
extern "C" fn version_full() -> Version {
    VERSION_FULL
}
