//! Mark a stored object as derived from another
//!
//! See [`DeriveFrom`].

use crate::{
    objects::{AnyObject, Stored},
    storage::Handle,
    Core,
};

/// Mark a store object as derived from another
pub trait DeriveFrom {
    /// Mark this object as derived from the other object provided
    fn derive_from(self, other: &Self, core: &mut Core) -> Self;
}

impl<T> DeriveFrom for Handle<T>
where
    Self: Into<AnyObject<Stored>>,
{
    fn derive_from(self, other: &Self, core: &mut Core) -> Self {
        core.layers
            .presentation
            .derive_object(other.clone().into(), self.clone().into());
        self
    }
}
