//! Append-only object storage
//!
//! So, why a custom data structure? Well, for two reasons:
//!
//! 1. No limitations on performance.
//! 2. Best possible convenience.
//!
//! Please note that I'm deliberately saving "no limitations" on performance. So
//! far, performance has not been a priority, so this might not be that fast.
//! But by having a custom data structure, we should be able to make performance
//! as good as we need it, within the limits of the practical.
//!
//! The second point, best possible convenience, is already realized.
//! [`Handle`]s can be owned, cloned, and dereference to the object they are
//! referencing. This is made possible by the append-only nature of our object
//! storage, and our immutable objects.
//!
//! There are other append-only data structures on `crates.io`. Some of them
//! look interesting, but none of them quite fit our needs and possibilities, so
//! a custom development seemed justified.
//!
//! But in any case, this was fun to write, and not that much work.

use std::{marker::PhantomData, sync::Arc};

use parking_lot::RwLock;

use super::{blocks::Blocks, Handle};

/// Append-only object storage
#[derive(Debug)]
pub struct Store<T> {
    inner: StoreInner<T>,
}

impl<T> Store<T> {
    /// Construct a new instance of `Store`
    pub fn new() -> Self {
        let block_size = 16384;
        Self {
            inner: Arc::new(RwLock::new(Blocks::new(block_size))),
        }
    }

    /// Insert an object into the store
    pub fn insert(&self, object: T) -> Handle<T> {
        let mut blocks = self.inner.write();
        let ptr = blocks.push(object);

        Handle {
            store: self.inner.clone(),
            ptr,
        }
    }

    /// Iterate over all objects in this store
    pub fn iter(&self) -> Iter<T> {
        Iter {
            store: self.inner.clone(),
            next_block: 0,
            next_object: 0,
            _a: PhantomData,
        }
    }

    /// Reserve a slot for an object
    ///
    /// Returns a [`Reservation`], which can be used to access the [`Handle`] of
    /// an object that hasn't been added yet. This makes it possible to use the
    /// [`Handle`]'s ID in the construction of the object, or to create groups
    /// of objects that reference each other through their [`Handle`]s.
    pub fn reserve(&self) -> Reservation<T> {
        let mut blocks = self.inner.write();
        let (index, ptr) = blocks.reserve();

        Reservation {
            store: self.inner.clone(),
            index,
            ptr,
        }
    }
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item = Handle<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over objects in a [`Store`]
pub struct Iter<'a, T> {
    store: StoreInner<T>,
    next_block: usize,
    next_object: usize,
    _a: PhantomData<&'a ()>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let blocks = self.store.read();

        let block = blocks.get(self.next_block)?;
        let object = block.get(self.next_object);

        self.next_object += 1;
        if self.next_object >= block.len() {
            self.next_block += 1;
        }

        Some(Handle {
            store: self.store.clone(),
            ptr: object,
        })
    }
}

/// A reservation of a slot for an object within a [`Store`]
///
/// See [`Store::reserve`].
pub struct Reservation<T> {
    store: StoreInner<T>,
    ptr: *mut Option<T>,
    index: (usize, usize),
}

impl<T> Reservation<T> {
    /// Access the [`Handle`] for this reservation
    ///
    /// You **must not** dereference the handle to access the object it
    /// references, until you initialized that object by calling
    /// [`Reservation::complete`]. Doing otherwise will lead to a panic.
    pub fn handle(&self) -> Handle<T> {
        Handle {
            store: self.store.clone(),
            ptr: self.ptr,
        }
    }

    /// Complete the reservation by providing an object
    ///
    /// This method consumes the reservation. After calling it, you can use any
    /// [`Handle`]s you acquired from [`Reservation::handle`] without
    /// limitations.
    pub fn complete(self, object: T) -> Handle<T> {
        let mut blocks = self.store.write();
        let ptr = blocks.insert(self.index, object);

        Handle {
            store: self.store.clone(),
            ptr,
        }
    }
}

pub type StoreInner<T> = Arc<RwLock<Blocks<T>>>;

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn insert_and_handle() {
        let store = Store::new();

        let object = 0;
        let handle = store.insert(object);

        assert_eq!(*handle, object);
    }

    #[test]
    fn insert_and_iter() {
        let store = Store::new();

        let a = store.insert(0);
        let b = store.insert(1);

        let objects = store.iter().collect::<Vec<_>>();
        assert_eq!(objects, [a, b])
    }

    #[test]
    fn reserve() {
        let store = Store::<i32>::new();

        let a = store.reserve();
        let b = store.reserve();

        let id_a = a.handle().id();
        let id_b = b.handle().id();
        assert_ne!(id_a, id_b);

        let a = a.complete(0);
        let b = b.complete(1);

        assert_eq!(*a, 0);
        assert_eq!(*b, 1);

        let objects = store.iter().collect::<Vec<_>>();
        assert_eq!(objects, [a, b]);
    }
}
