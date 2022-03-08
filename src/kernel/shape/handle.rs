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
