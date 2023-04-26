use std::{
    cmp::PartialEq,
    collections::{btree_set, BTreeSet},
};

use crate::{
    storage::Handle,
};

/// A collection of objects
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Set<T: PartialEq + Ord> {
    inner: BTreeSet<Handle<T>>,
}

impl<T: PartialEq + Ord> Default for Set<T> {
    fn default() -> Self {
        Self {
            inner: BTreeSet::default(),
        }
    }
}

impl<T: PartialEq + Ord> Set<T> {
    /// Create an empty instance of `Set`
    pub fn new() -> Self {
        Self::default()
    }

    /// Find the given object
    pub fn find(&self, other: &Handle<T>) -> Option<Handle<T>> {
        for f in self {
            if f == other {
                return Some(f.clone());
            }
        }

        None
    }
}

impl<T: PartialEq + Ord> Extend<Handle<T>> for Set<T> {
    fn extend<I: IntoIterator<Item = Handle<T>>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl<T: PartialEq + Ord> FromIterator<Handle<T>> for Set<T> {
    fn from_iter<I: IntoIterator<Item = Handle<T>>>(iter: I) -> Self {
        let mut items = Self::new();
        items.extend(iter);
        items
    }
}

impl<T: PartialEq + Ord> IntoIterator for Set<T> {
    type Item = Handle<T>;
    type IntoIter = btree_set::IntoIter<Handle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T: PartialEq + Ord> IntoIterator for &'a Set<T> {
    type Item = &'a Handle<T>;
    type IntoIter = btree_set::Iter<'a, Handle<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
