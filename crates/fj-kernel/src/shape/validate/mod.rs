mod coherence;
mod structural;
mod uniqueness;

pub use self::{
    coherence::{CoherenceIssues, CoherenceMismatch},
    structural::StructuralIssues,
    uniqueness::{DuplicateEdge, UniquenessIssues},
};

use fj_math::Scalar;

use crate::objects::{Curve, Cycle, Edge, Face, Surface, Vertex};

use super::{stores::Stores, Handle, Object};

pub trait Validate {
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        min_distance: Scalar,
        max_distance: Scalar,
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
        max_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        coherence::validate_edge(self, max_distance)?;
        structural::validate_edge(self, stores)?;
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Coherence validation failed
    #[error("Coherence validation failed")]
    Coherence(#[from] CoherenceIssues),

    /// Geometric validation failed
    #[error("Geometric validation failed")]
    Geometric,

    /// Structural validation failed
    #[error("Structural validation failed")]
    Structural(#[from] StructuralIssues),

    /// Uniqueness validation failed
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
    pub fn missing_vertex(&self, vertex: &Handle<Vertex>) -> bool {
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
