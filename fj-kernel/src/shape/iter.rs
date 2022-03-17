use std::{iter, slice};

use super::{
    handle::{Handle, Storage},
    Store,
};

/// An iterator over geometric or topological objects
///
/// Returned by various methods of the [`Shape`] API.
pub struct Iter<'r, T> {
    inner: Inner<'r, T>,
}

impl<'r, T> Iter<'r, T> {
    pub(super) fn new(store: &'r Store<T>) -> Self {
        let handle: fn(&Storage<T>) -> Handle<T> = |storage| storage.handle();
        let inner = store.iter().map(handle);
        Self { inner }
    }

    /// Convert this iterator over handles into an iterator over values
    ///
    /// This is a convenience method, for cases where no `Handle` is needed.
    pub fn values(self) -> impl Iterator<Item = T> + 'r
    where
        T: Clone,
    {
        self.inner.map(|handle| handle.get().clone())
    }
}

impl<T> Iterator for Iter<'_, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

type Inner<'r, T> =
    iter::Map<slice::Iter<'r, Storage<T>>, fn(&Storage<T>) -> Handle<T>>;
