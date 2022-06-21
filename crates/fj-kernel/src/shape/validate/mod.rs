mod uniqueness;

pub use self::uniqueness::{DuplicateEdge, UniquenessIssues};

use fj_math::Scalar;

use crate::{
    objects::{Curve, Cycle, Edge, Face, Surface, Vertex},
    validation::ValidationError,
};

use super::{stores::Stores, Handle, Object};

pub trait Validate {
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError>
    where
        Self: Object;
}

impl Validate for Curve<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Surface {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Vertex {
    /// Validate the vertex
    ///
    /// # Implementation note
    ///
    /// In the future, this method is likely to validate more than it already
    /// does. See documentation of [`crate::kernel`] for some context on that.
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        uniqueness::validate_vertex(self, handle, &stores.vertices)?;

        Ok(())
    }
}

impl Validate for Edge<3> {
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        uniqueness::validate_edge(self, handle, &stores.edges)?;

        Ok(())
    }
}

impl Validate for Cycle<3> {
    /// Validate the cycle
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Face {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;
