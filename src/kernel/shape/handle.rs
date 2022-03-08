use std::{hash::Hash, ops::Deref, rc::Rc};

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Storage<T>(Rc<T>);

impl<T> Storage<T> {
    pub(super) fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    pub(super) fn handle(&self) -> Handle<T> {
        Handle(self.clone())
    }
}

impl<T> Deref for Storage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> Clone for Storage<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

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
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handle<T>(Storage<T>);

impl<T> Handle<T> {
    /// Access the object that the handle references
    ///
    /// `Handle` also implements `Deref`, but as that can be inconvenient to use
    /// in some cases, this method is an inherent proxy for that.
    pub fn get(&self) -> &T {
        self.0.deref()
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
