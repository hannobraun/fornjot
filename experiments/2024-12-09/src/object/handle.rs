use std::{cmp::Ordering, fmt, ops::Deref, rc::Rc};

use super::{HandleAny, Object};

/// # A typed handle to an object
///
/// Handles provide a layer of identity to objects, enabling the same object to
/// be shared from multiple locations in the object graph.
///
/// Right now, this doesn't make much of a difference, but eventually it's going
/// to be important for various validation checks. (See the validation stuff in
/// the current mainline code for more information on that.)
///
/// The longer-term idea here, is to use this as a reference to an object that
/// is stored in a way that makes this object performant to access. Right now,
/// we just allocate all objects within [`Rc`] though, as a placeholder.
pub struct Handle<T> {
    inner: Rc<T>,
}

impl<T> Handle<T> {
    /// # Create a new handle
    ///
    /// Eventually, this type probably won't have a public constructor, and
    /// you'll create a `Handle` via some kind of collection/arena thing.
    ///
    /// For now, objects just live on the heap, in reference-counted ([`Rc`])
    /// allocations.
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<T> Handle<T>
where
    T: Object + 'static,
{
    /// # Create an untyped handle that refers to the same object
    pub fn to_any(&self) -> HandleAny {
        self.clone().into_any()
    }

    /// # Convert this handle into an untyped one that refers to the same object
    pub fn into_any(self) -> HandleAny {
        HandleAny { inner: self.inner }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Rc::as_ptr(&self.inner)
            .cast::<()>()
            .cmp(&Rc::as_ptr(&other.inner).cast::<()>())
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Handle")
            .field("inner", &Rc::as_ptr(&self.inner))
            .finish()
    }
}
