use crate::{
    objects::{SurfaceVertex, Vertex},
    stores::Stores,
};

use super::HasPartialForm;

/// Either a partial object or a full one
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaybePartial<T: HasPartialForm> {
    /// A full object
    Full(T),

    /// A partial object
    Partial(T::PartialForm),
}

impl<T: HasPartialForm> MaybePartial<T> {
    /// Return the full object, either directly or by building it
    pub fn into_full(self, stores: &Stores) -> T {
        match self {
            Self::Partial(partial) => T::from_partial(partial, stores),
            Self::Full(full) => full,
        }
    }

    /// Return the partial object, either directly or via conversion
    pub fn into_partial(self) -> T::PartialForm {
        match self {
            Self::Partial(partial) => partial,
            Self::Full(full) => full.into(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the surface form
    pub fn surface_form(&self) -> Option<&SurfaceVertex> {
        match self {
            Self::Full(full) => Some(full.surface_form()),
            Self::Partial(partial) => partial.surface_form.as_ref(),
        }
    }
}
