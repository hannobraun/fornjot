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

use std::{
    any::type_name, fmt, hash::Hash, marker::PhantomData, ops::Deref, sync::Arc,
};

use parking_lot::RwLock;

use super::blocks::Blocks;

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

/// A handle for an object
///
/// You can get an instance of `Handle` by inserting an object into a store. See
/// [`Store::insert`]. A handle dereferences to the object it points to, via its
/// [`Deref`] implementation.
///
/// # Equality and Identity
///
/// Equality of `Handle`s is defined via the objects they reference. If those
/// objects are equal, the `Handle`s are considered equal.
///
/// This is distinct from the *identity* of the referenced objects. Two objects
/// might be equal, but they might be have been created at different times, for
/// different reasons, and thus live in different slots in the storage. This is
/// a relevant distinction when validating objects, as equal but not identical
/// objects might be a sign of a bug.
///
/// You can compare the identity of two objects through their `Handle`s, by
/// comparing the values returned by [`Handle::id`].
pub struct Handle<T> {
    store: StoreInner<T>,
    ptr: *const Option<T>,
}

impl<T> Handle<T> {
    /// Access this pointer's unique id
    pub fn id(&self) -> ObjectId {
        ObjectId(self.ptr as u64)
    }

    /// Return a clone of the object this handle refers to
    pub fn clone_object(&self) -> T
    where
        T: Clone,
    {
        self.deref().clone()
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // `Handle` keeps a reference to `StoreInner`. Since that is an `Arc`
        // under the hood, we know that as long as an instance of `Handle`
        // exists, the `StoreInner` its data lives in is still alive. Even if
        // the `Store` was dropped.
        //
        // The `Store` API ensures two things:
        //
        // 1. That no `Handle` is ever created, until the object it references
        //    has at least been reserved.
        // 2. That the memory objects live in is never deallocated.
        //
        // That means that as long as a `Handle` exists, the object is
        // references has at least been reserved, and has not been deallocated.
        //
        // Given all this, we know that the following must be true:
        //
        // - The pointer is not null.
        // - The pointer is properly aligned.
        // - The pointer is dereferenceable.
        // - The pointer points to an initialized instance of `T`.
        //
        // Further, there is no way to (safely) get a `&mut` reference to any
        // object in a `Store`/`Block`. So we know that the aliasing rules for
        // the reference we return here are enforced.
        //
        // Furthermore, all of the code mentioned here is covered by unit tests,
        // which I've run successfully run under Miri.
        let cell = unsafe { &*self.ptr };

        // Can only happen, if the object has been reserved, but the reservation
        // was never completed.
        cell.as_ref()
            .expect("Handle references non-existing object")
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            ptr: self.ptr,
        }
    }
}

impl<T> Eq for Handle<T> where T: Eq {}

impl<T> PartialEq for Handle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T> Hash for Handle<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl<T> Ord for Handle<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<T> PartialOrd for Handle<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = {
            let type_name = type_name::<T>();
            match type_name.rsplit_once("::") {
                Some((_, name)) => name,
                None => type_name,
            }
        };
        let id = self.id().0;

        write!(f, "{name} @ {id:#x}")?;

        Ok(())
    }
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

/// Represents the ID of an object
///
/// See [`Handle::id`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ObjectId(u64);

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

type StoreInner<T> = Arc<RwLock<Blocks<T>>>;

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
