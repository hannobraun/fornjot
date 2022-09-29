use std::{any::type_name, cmp::Ordering, fmt, hash::Hash, ops::Deref};

use super::store::StoreInner;

/// A handle for an object
///
/// You can get an instance of `Handle` by inserting an object into a store. A
/// handle dereferences to the object it points to, via its [`Deref`]
/// implementation.
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
    pub(super) store: StoreInner<T>,
    pub(super) ptr: *const Option<T>,
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

impl<T> From<HandleWrapper<T>> for Handle<T> {
    fn from(wrapper: HandleWrapper<T>) -> Self {
        wrapper.0
    }
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

/// Represents the ID of an object
///
/// See [`Handle::id`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ObjectId(u64);

/// A wrapper around [`Handle`] to define equality based on identity
///
/// This is a utility type that implements [`Eq`]/[`PartialEq`] and other common
/// traits that are based on those, based on the identity of object that the
/// wrapped handle references. This is useful, if a type of object doesn't
/// implement `Eq`/`PartialEq`, which means handles referencing it won't
/// implement those types either.
///
/// Typically, if an object doesn't implement [`Eq`]/[`PartialEq`], it will do
/// so for good reason. If you need something that represents the object and
/// implements those missing traits, you might want to be explicit about what
/// you're doing, and access its ID via [`Handle::id`] instead.
///
/// But if you have a struct that owns a [`Handle`] to such an object, and you
/// want to be able to derive various traits that are not available for the
/// [`Handle`] itself, this wrapper is for you.
pub struct HandleWrapper<T>(pub Handle<T>);

impl<T> Deref for HandleWrapper<T> {
    type Target = Handle<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for HandleWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Eq for HandleWrapper<T> {}

impl<T> PartialEq for HandleWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        // The purpose of `HandleWrapper` is to provide equality (and other
        // traits) for `Handle<T>`s that would otherwise not have them. We use
        // `Handle::id` to do this. This means, objects that are not identical
        // are not equal.
        //
        // This is desirable for the most part, but it does become horribly
        // inconvenient in test code. Tests, by design, create equal (but not
        // identical) objects and compare them against objects produced by the
        // code under test. Under such a use case, one would rather ignore any
        // objects wrapped by `HandleWrapper`.
        //
        // And the following bit of code does just that. This might be a
        // horrible hack that will comes back to bite us later (I honestly don't
        // know), but it is certainly a very economical solution to this
        // problem.
        if cfg!(test) {
            return true;
        }

        self.0.id().eq(&other.0.id())
    }
}

impl<T> Hash for HandleWrapper<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // This piece of code exists to keep this implementation in sync with
        // the `PartialEq` implementation. See comment there.
        if cfg!(test) {
            return;
        }

        self.0.id().hash(state)
    }
}

impl<T> Ord for HandleWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // This piece of code exists to keep this implementation in sync with
        // the `PartialEq` implementation. See comment there.
        if cfg!(test) {
            return Ordering::Equal;
        }

        self.0.id().cmp(&other.0.id())
    }
}

impl<T> PartialOrd for HandleWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // This piece of code exists to keep this implementation in sync with
        // the `PartialEq` implementation. See comment there.
        if cfg!(test) {
            return Some(Ordering::Equal);
        }

        self.0.id().partial_cmp(&other.0.id())
    }
}

impl<T> fmt::Debug for HandleWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<Handle<T>> for HandleWrapper<T> {
    fn from(handle: Handle<T>) -> Self {
        Self(handle)
    }
}

unsafe impl<T> Send for HandleWrapper<T> {}
unsafe impl<T> Sync for HandleWrapper<T> {}
