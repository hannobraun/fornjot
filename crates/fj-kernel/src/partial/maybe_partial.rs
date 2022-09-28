use fj_math::Point;

use crate::{
    objects::{Curve, GlobalCurve, GlobalEdge, Surface, SurfaceVertex, Vertex},
    stores::{Handle, Stores},
};

use super::HasPartial;

/// Either a partial object or a full one
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaybePartial<T: HasPartial> {
    /// A full object
    Full(T),

    /// A partial object
    Partial(T::Partial),
}

impl<T: HasPartial> MaybePartial<T> {
    /// If this is a partial object, update it
    pub fn update_partial(
        self,
        f: impl FnOnce(T::Partial) -> T::Partial,
    ) -> Self {
        match self {
            Self::Partial(partial) => Self::Partial(f(partial)),
            _ => self,
        }
    }

    /// Return the full object, either directly or by building it
    pub fn into_full(self, stores: &Stores) -> T {
        match self {
            Self::Partial(partial) => T::from_partial(partial, stores),
            Self::Full(full) => full,
        }
    }

    /// Return the partial object, either directly or via conversion
    pub fn into_partial(self) -> T::Partial {
        match self {
            Self::Partial(partial) => partial,
            Self::Full(full) => full.into(),
        }
    }
}

impl<T> From<T> for MaybePartial<T>
where
    T: HasPartial,
{
    fn from(full: T) -> Self {
        Self::Full(full)
    }
}

// Unfortunately, we can't add a blanket implementation from `T::Partial` for
// `MaybePartial<T>`, as that would conflict.

impl MaybePartial<Curve> {
    /// Access the global form
    pub fn global_form(&self) -> Option<MaybePartial<Handle<GlobalCurve>>> {
        match self {
            Self::Full(full) => Some(full.global_form().clone().into()),
            Self::Partial(partial) => partial.global_form.clone(),
        }
    }
}

impl MaybePartial<GlobalEdge> {
    /// Access the curve
    pub fn curve(&self) -> Option<&Handle<GlobalCurve>> {
        match self {
            Self::Full(full) => Some(full.curve()),
            Self::Partial(partial) => partial.curve.as_ref(),
        }
    }
}

impl MaybePartial<SurfaceVertex> {
    /// Access the position
    pub fn position(&self) -> Option<Point<2>> {
        match self {
            Self::Full(full) => Some(full.position()),
            Self::Partial(partial) => partial.position,
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Option<&Surface> {
        match self {
            Self::Full(full) => Some(full.surface()),
            Self::Partial(partial) => partial.surface.as_ref(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the surface form
    pub fn surface_form(&self) -> Option<MaybePartial<SurfaceVertex>> {
        match self {
            Self::Full(full) => Some((*full.surface_form()).into()),
            Self::Partial(partial) => partial.surface_form.clone(),
        }
    }
}
