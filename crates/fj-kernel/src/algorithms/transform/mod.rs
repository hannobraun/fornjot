//! API for transforming objects

mod curve;
mod cycle;
mod edge;
mod face;
mod path;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

use fj_math::{Transform, Vector};

use crate::{
    objects::Objects,
    partial::{HasPartial, MaybePartial, Partial},
};

/// Transform an object
///
/// # Implementation Note
///
/// So far, a general `transform` method is available, along some convenience
/// methods for more specific transformations.
///
/// More convenience methods can be added as required. The only reason this
/// hasn't been done so far, is that no one has put in the work yet.
pub trait TransformObject: Sized {
    /// Transform the object
    #[must_use]
    fn transform(self, transform: &Transform, stores: &Objects) -> Self;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    #[must_use]
    fn translate(self, offset: impl Into<Vector<3>>, stores: &Objects) -> Self {
        self.transform(&Transform::translation(offset), stores)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    #[must_use]
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        stores: &Objects,
    ) -> Self {
        self.transform(&Transform::rotation(axis_angle), stores)
    }
}

impl<T> TransformObject for T
where
    T: HasPartial,
    T::Partial: TransformObject,
{
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        self.to_partial().transform(transform, stores).build(stores)
    }
}

impl<T> TransformObject for MaybePartial<T>
where
    T: HasPartial + TransformObject,
    T::Partial: TransformObject,
{
    fn transform(self, transform: &Transform, stores: &Objects) -> Self {
        match self {
            Self::Full(full) => Self::Full(full.transform(transform, stores)),
            Self::Partial(partial) => {
                Self::Partial(partial.transform(transform, stores))
            }
        }
    }
}
