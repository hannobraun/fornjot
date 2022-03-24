use std::collections::HashSet;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Vertex},
};

use super::Handle;

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;

/// An error that can occur during a validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[error("Structural validation failed")]
    Structural(StructuralIssues),

    /// Uniqueness validation failed
    ///
    /// Uniqueness validation checks, that an object is unique. Uniqueness is
    /// only required for topological objects, as there's no harm in geometric
    /// objects being duplicated.
    #[error("Uniqueness validation failed")]
    #[allow(unused)]
    Uniqueness,

    /// Geometric validation failed
    ///
    /// Geometric validation checks, that various geometric constraints of an
    /// object are upheld. For example, edges or faces might not be allowed to
    /// intersect.
    #[error("Geometric validation failed")]
    #[allow(unused)]
    Geometric,
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    #[cfg(test)]
    pub fn missing_curve(&self, curve: &Handle<Curve>) -> bool {
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
    pub fn missing_edge(&self, edge: &Handle<Edge>) -> bool {
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
    pub fn missing_cycle(&self, cycle: &Handle<Cycle>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}

impl From<StructuralIssues> for ValidationError {
    fn from(issues: StructuralIssues) -> Self {
        Self::Structural(issues)
    }
}

/// Structural issues found during validation
///
/// Used by [`ValidationError`].
#[derive(Debug, Default)]
pub struct StructuralIssues {
    /// Missing curve found in edge validation
    pub missing_curve: Option<Handle<Curve>>,

    /// Missing vertices found in edge validation
    pub missing_vertices: HashSet<Handle<Vertex>>,

    /// Missing edges found in cycle validation
    pub missing_edges: HashSet<Handle<Edge>>,

    /// Missing surface found in face validation
    pub missing_surface: Option<Handle<Surface>>,

    /// Missing cycles found in cycle validation
    pub missing_cycles: HashSet<Handle<Cycle>>,
}
