use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::HasPartial;

/// Wrapper around a partial object
///
/// Controls access to the partial object. Can be cloned, to access the same
/// partial object from multiple locations.
pub struct Partial<T: HasPartial> {
    inner: Inner<T>,
}

impl<T: HasPartial> Partial<T> {
    /// Construct a `Partial` with a default inner partial object
    pub fn new() -> Self {
        Self::from_partial(T::Partial::default())
    }

    /// Construct a `Partial` from a partial object
    pub fn from_partial(partial: T::Partial) -> Self {
        let inner = Inner::new(InnerObject { partial });
        Self { inner }
    }

    /// Access the partial object
    pub fn read(&self) -> impl Deref<Target = T::Partial> + '_ {
        RwLockReadGuard::map(self.inner.read(), |inner| &inner.partial)
    }

    /// Access the partial object mutably
    ///
    /// # Panics
    ///
    /// Panics, if this method is called while the return value from a previous
    /// call is still borrowed.
    pub fn write(&mut self) -> impl DerefMut<Target = T::Partial> + '_ {
        RwLockWriteGuard::map(self.inner.write(), |inner| &mut inner.partial)
    }
}

impl<T: HasPartial> Clone for Partial<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: HasPartial> Default for Partial<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct Inner<T: HasPartial>(Arc<RwLock<InnerObject<T>>>);

impl<T: HasPartial> Inner<T> {
    fn new(inner: InnerObject<T>) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }

    fn read(&self) -> RwLockReadGuard<InnerObject<T>> {
        self.0.read()
    }

    fn write(&self) -> RwLockWriteGuard<InnerObject<T>> {
        self.0
            .try_write()
            .expect("Tried to modify `Partial` that is already being modified")
    }
}

impl<T: HasPartial> Clone for Inner<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

struct InnerObject<T: HasPartial> {
    partial: T::Partial,
}
