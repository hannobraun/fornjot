//! Infrastructure for types that have a local and a global form

use fj_math::Point;

use crate::objects::Curve;

/// A wrapper around the local and global forms of a type
///
/// The local form is whatever representation of the value that is most
/// appropriate in a given local context, which might be a curve or surface. The
/// global form is the global 3D form of the same value.
///
/// The purpose of storing both forms is to be able to losslessly convert
/// between them. Even if this conversion can be computed on the fly, it might
/// be lossy due to floating point accuracy issues.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Local<T: LocalForm> {
    local: T,
    global: T::GlobalForm,
}

impl<T: LocalForm> Local<T> {
    /// Construct a new instance
    ///
    /// It is the caller's responsibility to make sure that the local and global
    /// forms passed into this constructor match.
    pub fn new(local: impl Into<T>, global: impl Into<T::GlobalForm>) -> Self {
        Self {
            local: local.into(),
            global: global.into(),
        }
    }

    /// Access the local form of the value
    pub fn local_form(&self) -> &T {
        &self.local
    }

    /// Access the global form of the value
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

impl LocalForm for Point<1> {
    type GlobalForm = Point<3>;
}

impl LocalForm for Point<2> {
    type GlobalForm = Point<3>;
}
