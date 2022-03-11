use crate::kernel::topology::{
    edges::{Cycle, Edge},
    faces::Face,
    vertices::Vertex,
};

use super::handle::Handle;

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
    type Structural = ();
}

impl Validatable for Cycle {
    type Structural = ();
}

impl Validatable for Face {
    type Structural = ();
}
