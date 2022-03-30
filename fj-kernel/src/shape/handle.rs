use std::hash::Hash;

use slotmap::DefaultKey;

use super::stores::Store;

/// A handle to an object stored within [`Shape`]
///
/// If an object of type `T` (this could be `Curve`, `Vertex`, etc.) is added to
/// `Shape`, a `Handle<T>` is returned. This handle is then used in topological
/// types to refer to the object, instead of having those types own an instance
/// of the object.
///
/// This approach has two advantages:
///
/// 1. The object can't be mutated through the handle. Since an object can be
///    referred to by multiple other objects, mutating it locally would have no
///    effect on those other references. `Handle` preventing that removes this
///    source of errors.
/// 2. The object is guaranteed to be in `Shape`, as `Handle`s can't be created
///    any other way. This means that if the `Shape` needs to be modified, any
///    objects can be updated once, without requiring an update of all the other
///    objects that reference it.
///
/// # Equality
///
/// The equality of [`Handle`] is very strictly defined in terms of identity.
/// Two [`Handle`]s are considered equal, if they refer to objects in the same
/// memory location.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handle<T> {
    key: DefaultKey,
    store: Store<T>,
}

impl<T> Handle<T> {
    pub(super) fn new(key: DefaultKey, store: Store<T>) -> Self {
        Self { key, store }
    }

    pub(super) fn key(&self) -> DefaultKey {
        self.key
    }

    pub(super) fn store(&self) -> &Store<T> {
        &self.store
    }

    /// Access the object that the handle references
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.store
            .read()
            .get(self.key)
            // Can't panic, unless the handle was invalid in the first place.
            // Objects are never removed from `Store`, so if we have a handle
            // pointing to it, it should be there.
            .unwrap()
            .clone()
    }
}
