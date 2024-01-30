use std::{
    any::type_name, borrow::Borrow, cmp::Ordering, fmt, hash::Hash, ops::Deref,
};

use super::{blocks::Index, store::StoreInner};

/// # A handle that references a stored object
///
/// You can get an instance of `Handle` by inserting an object into a store. A
/// handle dereferences to the object it points to, via its [`Deref`]
/// implementation.
///
/// ## Bare objects and stored objects
///
/// A bare object is just that: an instance of a bare object type. Once a bare
/// objects is inserted into storage, it becomes a stored object. A stored
/// object is owned by the store, and can be referenced through instances of
/// `Handle`.
///
/// The point of doing this, is to provide objects with a unique identity, via
/// their location within storage. The importance of this is expanded upon in
/// the next section.
///
/// ## Equality and Identity
///
/// Most objects have [`Eq`]/[`PartialEq`] implementations that can be used to
/// determine equality. Those implementations are derived, meaning two objects
/// are equal, if all of their fields are equal. This can be used to compare
/// objects structurally. [`Handle`]'s own [`Eq`]/[`PartialEq`] implementations
/// defer to those of the stored object it references.
///
/// However, that two objects are *equal* does not mean they are *identical*.
///
/// This distinction is relevant, because non-identical objects that are
/// *supposed* to be equal can in fact end up equal, if they are created based
/// on simple input data (as you might have in a unit test). But they might end
/// up slightly different, if they are created based on complex input data (as
/// you might have in a real-world scenario). This situation would most likely
/// result in a bug that is not easily caught in testing.
///
/// You can compare the identity of two `Handle`s, by comparing the values
/// returned by [`Handle::id`].
///
/// ### Validation Must Use Identity
///
/// To prevent situations where everything looks fine during development, but
/// you end up with a bug in production, any validation code that compares
/// objects and expects them to be the same, must do that comparison based on
/// identity, not equality. That way, this problem can never happen, because we
/// never expect non-identical objects to be equal.
pub struct Handle<T> {
    pub(super) store: StoreInner<T>,
    pub(super) index: Index,
    pub(super) ptr: *const Option<T>,
}

impl<T> Handle<T> {
    /// Access the object's unique id
    pub fn id(&self) -> ObjectId {
        ObjectId::from_ptr(self.ptr)
    }

    /// Return a bare object, which is a clone of the referenced stored object
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
        // That means that as long as a `Handle` exists, the object it
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
        // which I've run successfully under Miri.
        let slot = unsafe { &*self.ptr };

        // Can only panic, if the object has been reserved, but the reservation
        // was never completed.
        slot.as_ref()
            .expect("Handle references non-existing object")
    }
}

impl<T> Borrow<T> for Handle<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            index: self.index,
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
        self.deref().hash(state);
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

impl<T> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = {
            let type_name = type_name::<T>();
            match type_name.rsplit_once("::") {
                Some((_, name)) => name,
                None => type_name,
            }
        };
        let id = self.id().0;
        let object = self.deref();

        if f.alternate() {
            write!(f, "{name} @ {id:#x} => {object:#?}")?;
        } else {
            write!(f, "{name} @ {id:#x}")?;
        }

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
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ObjectId(pub(crate) u64);

impl ObjectId {
    pub(crate) fn from_ptr<T>(ptr: *const T) -> ObjectId {
        Self(ptr as u64)
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.0;
        write!(f, "object id {id:#x}")
    }
}

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
        self.0.id().eq(&other.0.id())
    }
}

impl<T> Hash for HandleWrapper<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id().hash(state);
    }
}

impl<T> Ord for HandleWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.id().cmp(&other.0.id())
    }
}

impl<T> PartialOrd for HandleWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Debug for HandleWrapper<T>
where
    T: fmt::Debug,
{
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
