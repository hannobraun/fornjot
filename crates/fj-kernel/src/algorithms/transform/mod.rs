//! API for transforming objects

mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

use fj_math::{Transform, Vector};

use crate::{
    insert::Insert,
    objects::Objects,
    partial::{HasPartial, MaybePartial, Partial},
    storage::Handle,
    validate::{Validate, ValidationError},
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
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError>;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn translate(
        self,
        offset: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        self.transform(&Transform::translation(offset), objects)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        self.transform(&Transform::rotation(axis_angle), objects)
    }
}

impl<T> TransformObject for Handle<T>
where
    T: HasPartial + Insert,
    T::Partial: TransformObject,
    ValidationError: From<<T as Validate>::Error>,
{
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        Ok(self
            .to_partial()
            .transform(transform, objects)?
            .build(objects)?
            .insert(objects)?)
    }
}

impl<T> TransformObject for MaybePartial<T>
where
    T: HasPartial,
    Handle<T>: TransformObject,
    T::Partial: TransformObject,
{
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        match self {
            Self::Full(full) => {
                Ok(Self::Full(full.transform(transform, objects)?))
            }
            Self::Partial(partial) => {
                Ok(Self::Partial(partial.transform(transform, objects)?))
            }
        }
    }
}
