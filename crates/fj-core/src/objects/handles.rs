use std::{collections::BTreeSet, fmt::Debug};

use itertools::Itertools;

use crate::storage::Handle;

/// An ordered set of object handles
///
/// This is the data structure used by all objects that reference multiple
/// objects of the same type. It is a set, not containing any duplicate
/// elements, and it maintains the insertion order of those elements.
///
/// `HandleSet` implement `FromIterator`, but it must never be constructed from
/// an iterator that contains duplicate handles. This will result in a panic.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handles<T> {
    // This is supposed to be a set data structure, so what is that `Vec` doing
    // here? Well, it's here because we need it to preserve insertion order, but
    // that doesn't explain why it is here *alone*.
    //
    // If you look closely, you'll notice that this is an immutable data
    // structure (since it is used in objects, and objects themselves are
    // immutable). We make sure there are no duplicates when this is
    // constructed (see the `FromIterator` implementation below), but after
    // that, we're fine.
    inner: Vec<Handle<T>>,
}

impl<T> Handles<T> {
    /// Create a new instances of `Handles` from an iterator over `Handle<T>`
    ///
    /// # Panics
    ///
    /// Panics, if the iterator contains duplicate `Handle`s.
    pub fn new(handles: impl IntoIterator<Item = Handle<T>>) -> Self
    where
        T: Debug + Ord,
    {
        let mut added = BTreeSet::new();
        let mut inner = Vec::new();

        for handle in handles {
            if added.contains(&handle) {
                panic!(
                    "Constructing `HandleSet` with duplicate handle: {:?}",
                    handle
                );
            }

            added.insert(handle.clone());
            inner.push(handle);
        }

        Self { inner }
    }

    /// Return the number of handles in this set
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Indicate whether the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Return the n-th item
    pub fn nth(&self, index: usize) -> Option<&Handle<T>> {
        self.inner.get(index)
    }

    /// Return the n-th item, treating the index space as circular
    ///
    /// If the length of `Handles` is `i`, then retrieving the i-th edge using
    /// this method, is the same as retrieving the 0-th one.
    pub fn nth_circular(&self, index: usize) -> &Handle<T> {
        let index = index % self.len();
        self.nth(index)
            .expect("Index must be valid, due to modulo above")
    }

    /// Return the index of the item, if available
    pub fn index_of(&self, handle: &Handle<T>) -> Option<usize> {
        self.inner.iter().position(|h| h.id() == handle.id())
    }

    /// Access the item after the provided one
    ///
    /// Returns `None`, if the provided item is not in this iterator.
    pub fn after(&self, handle: &Handle<T>) -> Option<&Handle<T>> {
        self.index_of(handle)
            .map(|index| self.nth_circular(index + 1))
    }

    /// Access an iterator over the handles
    pub fn iter(&self) -> HandleIter<T> {
        HandleIter {
            handles: self,
            next_index: 0,
        }
    }

    /// Return iterator over the pairs of all handles
    pub fn pairs(&self) -> impl Iterator<Item = (&Handle<T>, &Handle<T>)> {
        self.iter().circular_tuple_windows()
    }
}

impl<O> FromIterator<Handle<O>> for Handles<O>
where
    O: Debug + Ord,
{
    fn from_iter<T: IntoIterator<Item = Handle<O>>>(handles: T) -> Self {
        Self::new(handles)
    }
}

/// An iterator over handles to objects
///
/// This struct is returned by the respective methods of all objects that
/// reference multiple objects of the same type.
pub struct HandleIter<'r, T> {
    handles: &'r Handles<T>,
    next_index: usize,
}

impl<'r, T> HandleIter<'r, T> {
    /// Return the n-th item
    ///
    /// This method is unaffected by any previous calls to `next`.
    pub fn nth(&self, index: usize) -> Option<&Handle<T>> {
        self.handles.nth(index)
    }

    /// Return the n-th item, treating the iterator as circular
    ///
    /// If the length of the iterator is `i`, then retrieving the i-th edge
    /// using this method, is the same as retrieving the 0-th one.
    ///
    /// This method is unaffected by any previous calls to `next`.
    pub fn nth_circular(&self, index: usize) -> &Handle<T> {
        self.handles.nth_circular(index)
    }

    /// Return the index of the item, if it is in this iterator
    ///
    /// This method is unaffected by any previous calls to `next`.
    pub fn index_of(&self, handle: &Handle<T>) -> Option<usize> {
        self.handles.index_of(handle)
    }

    /// Access the item after the provided one
    ///
    /// Returns `None`, if the provided item is not in this iterator.
    pub fn after(&self, handle: &Handle<T>) -> Option<&Handle<T>> {
        self.handles.after(handle)
    }

    /// Return iterator over the pairs of the remaining items in this iterator
    pub fn pairs(self) -> impl Iterator<Item = (&'r Handle<T>, &'r Handle<T>)> {
        self.handles.pairs()
    }
}

impl<'r, T> Iterator for HandleIter<'r, T> {
    // You might wonder why we're returning references to handles here, when
    // `Handle` already is kind of reference, and easily cloned.
    //
    // Most of the time that doesn't make a difference, but there are use cases
    // where dealing with owned `Handle`s is inconvenient, for example when
    // using iterator adapters. You can't return a reference to the argument of
    // an adapter's closure, if you own that argument. You can, if you just
    // reference the argument.
    type Item = &'r Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let handle = self.handles.inner.get(self.next_index);
        self.next_index += 1;
        handle
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.handles.inner.len();
        (size, Some(size))
    }
}

impl<T> ExactSizeIterator for HandleIter<'_, T> {}

// Deriving won't work, as that only derives `Clone` where `T: Clone`. But
// `HandleIter` can be `Clone`d unconditionally.
impl<T> Clone for HandleIter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            handles: self.handles,
            next_index: self.next_index,
        }
    }
}
