use std::{
    any::type_name,
    collections::BTreeMap,
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use type_map::TypeMap;

use crate::{
    insert::Insert,
    objects::Objects,
    partial::traits::PartialObject,
    services::Service,
    storage::{Handle, ObjectId},
};

use super::HasPartial;

/// Wrapper around a partial object
///
/// Controls access to the partial object. Can be cloned, to access the same
/// partial object from multiple locations.
pub struct Partial<T: HasPartial> {
    inner: Inner<T>,
}

impl<T: HasPartial + 'static> Partial<T> {
    /// Construct a `Partial` with a default inner partial object
    pub fn new(objects: &mut Service<Objects>) -> Self {
        Self::from_partial(T::Partial::new(objects))
    }

    /// Construct a `Partial` from a partial object
    pub fn from_partial(partial: T::Partial) -> Self {
        let inner = Inner::new(InnerObject {
            partial,
            full: None,
        });
        Self { inner }
    }

    /// Construct a partial from a full object
    pub fn from_full(full: Handle<T>, cache: &mut FullToPartialCache) -> Self {
        let inner = match cache.get(&full) {
            Some(inner) => inner,
            None => {
                let inner = Inner::new(InnerObject {
                    partial: T::Partial::from_full(&full, cache),
                    full: Some(full.clone()),
                });

                cache.insert(&full, inner.clone());

                inner
            }
        };

        Self { inner }
    }

    /// Access the ID of this partial object
    pub fn id(&self) -> ObjectId {
        self.inner.id()
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
    /// call to this method of [`Self::read`] is still borrowed.
    pub fn write(&mut self) -> impl DerefMut<Target = T::Partial> + '_ {
        let mut inner = self.inner.write();

        // If we created this partial object from a full one and then modify it,
        // it should not map back to the full object when calling `build`.
        inner.full = None;

        RwLockWriteGuard::map(inner, |inner| &mut inner.partial)
    }

    /// Build a full object from this partial one
    ///
    /// # Panics
    ///
    /// Panics, if a call to [`Self::write`] would panic.
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

impl<T: HasPartial + 'static> fmt::Debug for Partial<T>
where
    T::Partial: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = {
            let type_name = type_name::<T::Partial>();
            match type_name.rsplit_once("::") {
                Some((_, name)) => name,
                None => type_name,
            }
        };
        let id = self.id().0;
        let object = self.read().clone();

        if f.alternate() {
            write!(f, "{name} @ {id:#x} => {object:#?}")?;
        } else {
            write!(f, "{name} @ {id:#x}")?;
        }

        Ok(())
    }
}

impl<T: HasPartial> Clone for Partial<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: HasPartial + 'static> From<Handle<T>> for Partial<T> {
    fn from(full: Handle<T>) -> Self {
        let mut cache = FullToPartialCache::default();
        Self::from_full(full, &mut cache)
    }
}

#[derive(Debug)]
struct Inner<T: HasPartial>(Arc<RwLock<InnerObject<T>>>);

impl<T: HasPartial> Inner<T> {
    fn new(inner: InnerObject<T>) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }

    fn id(&self) -> ObjectId {
        ObjectId::from_ptr(Arc::as_ptr(&self.0))
    }

    fn read(&self) -> RwLockReadGuard<InnerObject<T>> {
        self.0
            .try_read()
            .expect("Tried to read `Partial` that is currently being modified")
    }

    fn write(&self) -> RwLockWriteGuard<InnerObject<T>> {
        self.0
            .try_write()
            .expect("Tried to modify `Partial` that is currently locked")
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

/// Caches conversions from full to partial objects
///
/// When creating a whole graph of partial objects from a graph of full ones,
/// the conversions must be cached, to ensure that each full object maps to
/// exactly one partial object.
///
/// Used by [`Partial::from_full`] and [`PartialObject::from_full`].
#[derive(Default)]
pub struct FullToPartialCache(TypeMap);

impl FullToPartialCache {
    fn get<T>(&mut self, handle: &Handle<T>) -> Option<Inner<T>>
    where
        T: HasPartial + 'static,
    {
        self.map().get(&handle.id()).cloned()
    }

    fn insert<T>(&mut self, handle: &Handle<T>, inner: Inner<T>)
    where
        T: HasPartial + 'static,
    {
        self.map().insert(handle.id(), inner);
    }

    fn map<T>(&mut self) -> &mut BTreeMap<ObjectId, Inner<T>>
    where
        T: HasPartial + 'static,
    {
        self.0
            .entry::<BTreeMap<ObjectId, Inner<T>>>()
            .or_insert_with(BTreeMap::new)
    }
}
