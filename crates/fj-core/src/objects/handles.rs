use std::{collections::BTreeSet, fmt::Debug};

use crate::storage::Handle;

/// An ordered set of object handles
///
/// This is an internal data structure that is used within objects that
/// reference multiple other objects of the same type. It does not contain any
/// duplicate elements, and it maintains the insertion order of those elements.
///
/// `HandleSet` implement `FromIterator`, but it must never be constructed from
/// an iterator that contains duplicate handles. This will result in a panic.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HandleSet<T> {
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

impl<T> HandleSet<T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn nth(&self, index: usize) -> Option<&Handle<T>> {
        self.inner.get(index)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Handle<T>> + Clone + ExactSizeIterator {
        self.inner.iter()
    }
}

impl<O> FromIterator<Handle<O>> for HandleSet<O>
where
    O: Debug + Ord,
{
    fn from_iter<T: IntoIterator<Item = Handle<O>>>(handles: T) -> Self {
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
}
