use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, SurfaceVertex, Vertex,
    },
    storage::Handle,
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
    Full(T),

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
    pub fn into_full(self, objects: &Objects) -> T {
        match self {
            Self::Partial(partial) => partial.build(objects),
            Self::Full(full) => full,
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

impl MaybePartial<Handle<Curve>> {
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

impl MaybePartial<Handle<GlobalEdge>> {
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
    pub fn vertices(&self) -> Option<&[Handle<GlobalVertex>; 2]> {
        match self {
            Self::Full(full) => {
                Some(full.vertices().access_in_normalized_order())
            }
            Self::Partial(partial) => partial.vertices.as_ref(),
        }
    }
}

impl MaybePartial<Handle<HalfEdge>> {
    /// Access the back vertex
    pub fn back(&self) -> Option<MaybePartial<Handle<Vertex>>> {
        match self {
            Self::Full(full) => Some(full.back().clone().into()),
            Self::Partial(partial) => {
                let [back, _] = &partial.vertices;
                back.clone()
            }
        }
    }

    /// Access the front vertex
    pub fn front(&self) -> Option<MaybePartial<Handle<Vertex>>> {
        match self {
            Self::Full(full) => Some(full.front().clone().into()),
            Self::Partial(partial) => {
                let [_, front] = &partial.vertices;
                front.clone()
            }
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> [Option<MaybePartial<Handle<Vertex>>>; 2] {
        match self {
            Self::Full(full) => {
                full.vertices().clone().map(|vertex| Some(vertex.into()))
            }
            Self::Partial(partial) => partial.vertices.clone(),
        }
    }
}

impl MaybePartial<Handle<SurfaceVertex>> {
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

impl MaybePartial<Handle<Vertex>> {
    /// Access the surface form
    pub fn surface_form(&self) -> Option<MaybePartial<Handle<SurfaceVertex>>> {
        match self {
            Self::Full(full) => Some(full.surface_form().clone().into()),
            Self::Partial(partial) => partial.surface_form.clone(),
        }
    }
}
