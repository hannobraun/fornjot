use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    insert::Insert, objects::Objects, partial2::traits::PartialObject,
    services::Service, storage::Handle,
};

use super::HasPartial;

/// Wrapper around a partial object
///
/// Controls access to the partial object. Can be cloned, to access the same
/// partial object from multiple locations.
#[derive(Debug)]
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
        let inner = Inner::new(InnerObject {
            partial,
            full: None,
        });
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

    /// Build a full object from this partial one
    ///
    /// # Panics
    ///
    /// Panics, if a return value of [`Self::write`] is still borrowed.
    pub fn build(self, objects: &mut Service<Objects>) -> Handle<T>
    where
        T: Insert,
    {
        let mut inner = self.inner.write();

        // If another instance of this `Partial` has already been built, re-use
        // the resulting full object.
        let partial = inner.partial.clone();
        let full = inner
            .full
            .get_or_insert_with(|| partial.build(objects).insert(objects));

        full.clone()
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

#[derive(Debug)]
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

#[derive(Debug)]
struct InnerObject<T: HasPartial> {
    partial: T::Partial,
    full: Option<Handle<T>>,
}
