use std::collections::HashSet;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::Handle;

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError<T>>;

/// An error that can occur during a validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError<T: Validatable> {
    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[error("Structural validation failed")]
    Structural(T::Structural),

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

impl ValidationError<Edge> {
    /// Indicate whether validation found a missing curve
    #[cfg(test)]
    pub fn missing_curve(&self, curve: &Handle<Curve>) -> bool {
        if let Self::Structural(missing) = self {
            return missing.0.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    #[cfg(test)]
    pub fn missing_vertex(&self, vertex: &Handle<Vertex>) -> bool {
        if let Self::Structural(missing) = self {
            return missing.1.contains(vertex);
        }

        false
    }
}

impl ValidationError<Cycle> {
    /// Indicate whether validation found a missing edge
    #[cfg(test)]
    pub fn missing_edge(&self, vertex: &Handle<Edge>) -> bool {
        if let Self::Structural(missing) = self {
            return missing.contains(vertex);
        }

        false
    }
}

impl ValidationError<Face> {
    /// Indicate whether validation found a missing surface
    #[cfg(test)]
    pub fn missing_surface(&self, surface: &Handle<Surface>) -> bool {
        if let Self::Structural(missing) = self {
            return missing.0.as_ref() == Some(surface);
        }

        false
    }

    /// Indicate whether validation found a missing cycle
    #[cfg(test)]
    pub fn missing_cycle(&self, cycle: &Handle<Cycle>) -> bool {
        if let Self::Structural(missing) = self {
            return missing.1.contains(cycle);
        }

        false
    }
}

/// Implemented for topological types, which can be validated
///
/// Used by [`ValidationError`] to provide context on how validation failed.
pub trait Validatable {
    type Structural;
}

impl Validatable for Vertex {
    type Structural = ();
}

impl Validatable for Edge {
    type Structural = (Option<Handle<Curve>>, HashSet<Handle<Vertex>>);
}

impl Validatable for Cycle {
    type Structural = HashSet<Handle<Edge>>;
}

impl Validatable for Face {
    type Structural = (Option<Handle<Surface>>, HashSet<Handle<Cycle>>);
}
