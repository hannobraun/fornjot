//! Infrastructure for types that have a local and a global form

use crate::objects::Curve;

/// A reference to an object, which includes a local form
///
/// This type is used by topological objects to reference other objects, while
/// also keeping track of a local representation of that object, which is often
/// more appropriate for various tasks.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Local<T: LocalForm> {
    local: T,
    global: T::GlobalForm,
}

impl<T: LocalForm> Local<T> {
    /// Construct a new instance of `LocalForm`
    ///
    /// It is the caller's responsibility to make sure that the local and
    /// canonical forms passed to this method actually match.
    pub fn new(local: T, global: T::GlobalForm) -> Self {
        Self { local, global }
    }

    /// Access the local form of the referenced object
    pub fn local(&self) -> &T {
        &self.local
    }

    /// Access the canonical form of the referenced object
    pub fn global(&self) -> &T::GlobalForm {
        &self.global
    }
}

/// Implemented for types that are the local form of a global type
///
/// See [`Local`] for more information.
pub trait LocalForm {
    /// The global form of the implementing type
    type GlobalForm;
}

impl LocalForm for Curve<2> {
    type GlobalForm = Curve<3>;
}
