use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, SurfaceVertex, Vertex,
    },
    path::SurfacePath,
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
    /// Indicate whether this is a full object
    pub fn is_full(&self) -> bool {
        if let Self::Full(_) = self {
            return true;
        }

        false
    }

    /// Indicate whether this is a partial object
    pub fn is_partial(&self) -> bool {
        if let Self::Partial(_) = self {
            return true;
        }

        false
    }

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

    /// Merge this `MaybePartial` with another of the same type
    pub fn merge_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Full(_), Self::Full(_)) => {
                panic!("Can't merge two full objects")
            }
            (Self::Full(full), Self::Partial(_))
            | (Self::Partial(_), Self::Full(full)) => Self::Full(full),
            (Self::Partial(a), Self::Partial(b)) => {
                Self::Partial(a.merge_with(b))
            }
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
    /// Access the path
    pub fn path(&self) -> Option<SurfacePath> {
        match self {
            MaybePartial::Full(full) => Some(full.path()),
            MaybePartial::Partial(partial) => partial.path(),
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Option<Handle<Surface>> {
        match self {
            MaybePartial::Full(full) => Some(full.surface().clone()),
            MaybePartial::Partial(partial) => partial.surface(),
        }
    }

    /// Access the global form
    pub fn global_form(&self) -> Option<MaybePartial<GlobalCurve>> {
        match self {
            Self::Full(full) => Some(full.global_form().clone().into()),
            Self::Partial(partial) => partial.global_form(),
        }
    }
}

impl MaybePartial<GlobalEdge> {
    /// Access the curve
    pub fn curve(&self) -> MaybePartial<GlobalCurve> {
        match self {
            Self::Full(full) => full.curve().clone().into(),
            Self::Partial(partial) => partial.curve(),
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> Option<[MaybePartial<GlobalVertex>; 2]> {
        match self {
            Self::Full(full) => Some(
                full.vertices().access_in_normalized_order().map(Into::into),
            ),
            Self::Partial(partial) => partial.vertices(),
        }
    }
}

impl MaybePartial<HalfEdge> {
    /// Access the front vertex
    pub fn front(&self) -> MaybePartial<Vertex> {
        match self {
            Self::Full(full) => full.front().clone().into(),
            Self::Partial(partial) => {
                let [_, front] = &partial.vertices();
                front.clone()
            }
        }
    }

    /// Access the vertices
    pub fn vertices(&self) -> [MaybePartial<Vertex>; 2] {
        match self {
            Self::Full(full) => full.vertices().clone().map(Into::into),
            Self::Partial(partial) => partial.vertices(),
        }
    }
}

impl MaybePartial<SurfaceVertex> {
    /// Access the position
    pub fn position(&self) -> Option<Point<2>> {
        match self {
            Self::Full(full) => Some(full.position()),
            Self::Partial(partial) => partial.position(),
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Option<Handle<Surface>> {
        match self {
            Self::Full(full) => Some(full.surface().clone()),
            Self::Partial(partial) => partial.surface(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the surface form
    pub fn surface_form(&self) -> MaybePartial<SurfaceVertex> {
        match self {
            Self::Full(full) => full.surface_form().clone().into(),
            Self::Partial(partial) => partial.surface_form(),
        }
    }
}
