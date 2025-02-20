//! Mark a stored object as derived from another
//!
//! See [`DeriveFrom`].

use crate::{
    Core,
    storage::Handle,
    topology::{AnyObject, Stored},
};

/// Mark a store object as derived from another
pub trait DeriveFrom {
    /// Mark this object as derived from the other object provided
    fn derive_from(self, original: &Self, core: &mut Core) -> Self;
}

impl<T> DeriveFrom for Handle<T>
where
    Self: Into<AnyObject<Stored>>,
{
    fn derive_from(self, original: &Self, core: &mut Core) -> Self {
        core.layers
            .presentation
            .derive_object(original.clone().into(), self.clone().into());
        self
    }
}
