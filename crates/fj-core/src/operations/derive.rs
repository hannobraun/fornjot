//! Mark a stored object as derived from another
//!
//! See [`DeriveFrom`].

use crate::{storage::Handle, Core};

/// Mark a store object as derived from another
pub trait DeriveFrom {
    /// Mark this object as derived from the other object provided
    fn derive_from(self, other: &Self, core: &mut Core) -> Self;
}

impl<T> DeriveFrom for Handle<T> {
    fn derive_from(self, _other: &Self, _core: &mut Core) -> Self {
        // This is currently a no-op. Eventually, it will trigger a command to
        // the layers that this information is relevant for.
        self
    }
}
