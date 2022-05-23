mod structural;
mod uniqueness;

pub use self::{structural::StructuralIssues, uniqueness::UniquenessIssues};

use fj_math::{Point, Scalar};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
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

impl Validate for Point<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
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

impl Validate for Vertex<3> {
    /// Validate the vertex
    ///
    /// # Implementation note
    ///
    /// In the future, this method is likely to validate more than it already
    /// does. See documentation of [`crate::kernel`] for some context on that.
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        structural::validate_vertex(self, stores)?;
        uniqueness::validate_vertex(self, handle, min_distance, stores)?;

        Ok(())
    }
}

impl Validate for Edge<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        structural::validate_edge(self, stores)?;
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
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        structural::validate_cycle(self, stores)?;
        Ok(())
    }
}

impl Validate for Face {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        structural::validate_face(self, stores)?;
        Ok(())
    }
}

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;

/// An error that can occur during a validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Geometric validation failed
    ///
    /// Geometric validation checks, that various geometric constraints of an
    /// object are upheld. For example, edges or faces might not be allowed to
    /// intersect.
    #[error("Geometric validation failed")]
    Geometric,

    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[error("Structural validation failed")]
    Structural(#[from] StructuralIssues),

    /// Uniqueness validation failed
    ///
    /// Uniqueness validation checks, that an object is unique. Uniqueness is
    /// only required for topological objects, as there's no harm in geometric
    /// objects being duplicated.
    #[error("Uniqueness validation failed")]
    Uniqueness(#[from] UniquenessIssues),
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    #[cfg(test)]
    pub fn missing_curve(&self, curve: &Handle<Curve<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_curve, .. }) = self {
            return missing_curve.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    #[cfg(test)]
    pub fn missing_vertex(&self, vertex: &Handle<Vertex<3>>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_vertices, ..
        }) = self
        {
            return missing_vertices.contains(vertex);
        }

        false
    }

    /// Indicate whether validation found a missing edge
    #[cfg(test)]
    pub fn missing_edge(&self, edge: &Handle<Edge<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_edges, .. }) = self {
            return missing_edges.contains(edge);
        }

        false
    }

    /// Indicate whether validation found a missing surface
    #[cfg(test)]
    pub fn missing_surface(&self, surface: &Handle<Surface>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_surface, ..
        }) = self
        {
            return missing_surface.as_ref() == Some(surface);
        }

        false
    }

    /// Indicate whether validation found a missing cycle
    #[cfg(test)]
    pub fn missing_cycle(&self, cycle: &Handle<Cycle<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}
