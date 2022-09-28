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

use crate::stores::Stores;

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
    /// The type of the transformed object
    ///
    /// # Implementation Note
    ///
    /// This type is temporary, while we transition to a global object store. It
    /// should be removed, once that transition is complete.
    type Transformed;

    /// Transform the object
    #[must_use]
    fn transform(
        self,
        transform: &Transform,
        stores: &Stores,
    ) -> Self::Transformed;

    /// Translate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    #[must_use]
    fn translate(
        self,
        offset: impl Into<Vector<3>>,
        stores: &Stores,
    ) -> Self::Transformed {
        self.transform(&Transform::translation(offset), stores)
    }

    /// Rotate the object
    ///
    /// Convenience wrapper around [`TransformObject::transform`].
    #[must_use]
    fn rotate(
        self,
        axis_angle: impl Into<Vector<3>>,
        stores: &Stores,
    ) -> Self::Transformed {
        self.transform(&Transform::rotation(axis_angle), stores)
    }
}
