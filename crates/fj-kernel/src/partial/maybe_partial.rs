use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, SurfaceVertex, Vertex,
    },
    storage::Handle,
    validate::ValidationError,
};

use super::{HasPartial, Partial};

/// Can be used everywhere either a partial or full objects are accepted
///
/// Some convenience methods are available for specific instances of
/// `MaybePartial` (like, `MaybePartial<Curve>`, or `MaybePartial<Vertex>`).
///
/// # Implementation Note
///
/// The set of available convenience methods is far from complete. Please feel
/// free to just add more, if you need them.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaybePartial<T: HasPartial> {
    /// A full object
    Full(Handle<T>),

    /// A partial object
    Partial(T::Partial),
}

impl<T: HasPartial> MaybePartial<T> {
    /// If this is a partial object, update it
    ///
    /// This is useful whenever a partial object can infer something about its
    /// parts from other parts, and wants to update what was inferred, in case
    /// it *can* be updated.
    pub fn update_partial(
        self,
        f: impl FnOnce(T::Partial) -> T::Partial,
    ) -> Self {
        match self {
            Self::Partial(partial) => Self::Partial(f(partial)),
            _ => self,
        }
    }

    /// Return or build a full object
    ///
    /// If this already is a full object, it is returned. If this is a partial
    /// object, the full object is built from it, using [`Partial::build`].
    pub fn into_full(
        self,
        objects: &Objects,
    ) -> Result<Handle<T>, ValidationError> {
        match self {
            Self::Partial(partial) => partial.build(objects),
            Self::Full(full) => Ok(full),
        }
    }

    /// Return or convert a partial object
    ///
    /// If this already is a partial object, is is returned. If this is a full
    /// object, it is converted into a partial object using
    /// [`HasPartial::to_partial`].
    pub fn into_partial(self) -> T::Partial {
        match self {
            Self::Partial(partial) => partial,
            Self::Full(full) => full.to_partial(),
        }
    }
}

impl<T> Default for MaybePartial<T>
where
    T: HasPartial,
    T::Partial: Default,
{
    fn default() -> Self {
        Self::Partial(T::Partial::default())
    }
}

impl<T> From<Handle<T>> for MaybePartial<T>
where
    T: HasPartial,
{
    fn from(full: Handle<T>) -> Self {
        Self::Full(full)
    }
}

// Unfortunately, we can't add a blanket implementation from `T::Partial` for
// `MaybePartial<T>`, as that would conflict.

impl MaybePartial<Curve> {
    /// Access the global form
    pub fn global_form(&self) -> Option<Handle<GlobalCurve>> {
        match self {
            Self::Full(full) => Some(full.global_form().clone()),
            Self::Partial(partial) => {
                partial.global_form.clone().map(Into::into)
            }
        }
    }
}

impl MaybePartial<GlobalEdge> {
    /// Access the curve
    pub fn curve(&self) -> Option<&Handle<GlobalCurve>> {
        match self {
            Self::Full(full) => Some(full.curve()),
            Self::Partial(partial) => {
                partial.curve.as_ref().map(|wrapper| &wrapper.0)
            }
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> Option<[MaybePartial<GlobalVertex>; 2]> {
        match self {
            Self::Full(full) => Some(
                full.vertices().access_in_normalized_order().map(Into::into),
            ),
            Self::Partial(partial) => partial.vertices.clone(),
        }
    }
}

impl MaybePartial<HalfEdge> {
    /// Access the front vertex
    pub fn front(&self) -> MaybePartial<Vertex> {
        match self {
            Self::Full(full) => full.front().clone().into(),
            Self::Partial(partial) => {
                let [_, front] = &partial.vertices;
                front.clone()
            }
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> [MaybePartial<Vertex>; 2] {
        match self {
            Self::Full(full) => full.vertices().clone().map(Into::into),
            Self::Partial(partial) => partial.vertices.clone(),
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
    pub fn surface(&self) -> Option<&Handle<Surface>> {
        match self {
            Self::Full(full) => Some(full.surface()),
            Self::Partial(partial) => partial.surface.as_ref(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the surface form
    pub fn surface_form(&self) -> MaybePartial<SurfaceVertex> {
        match self {
            Self::Full(full) => full.surface_form().clone().into(),
            Self::Partial(partial) => partial.surface_form.clone(),
        }
    }
}
