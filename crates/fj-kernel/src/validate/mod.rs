//! Infrastructure for validating objects

mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

pub use self::{
    cycle::CycleValidationError,
    edge::HalfEdgeValidationError,
    face::FaceValidationError,
    vertex::{SurfaceVertexValidationError, VertexValidationError},
};

use std::convert::Infallible;

use fj_math::Scalar;

/// Validate an object
///
/// This trait is used automatically when inserting an object into a store.
pub trait Validate: Sized {
    /// The error that validation of the implementing type can result in
    type Error: Into<ValidationError>;

    /// Validate the object using default configuration
    fn validate(&self) -> Result<(), Self::Error> {
        self.validate_with_config(&ValidationConfig::default())
    }

    /// Validate the object
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error>;
}

/// Configuration required for the validation process
#[derive(Debug, Clone, Copy)]
pub struct ValidationConfig {
    /// The minimum distance between distinct objects
    ///
    /// Objects whose distance is less than the value defined in this field, are
    /// considered identical.
    pub distinct_min_distance: Scalar,

    /// The maximum distance between identical objects
    ///
    /// Objects that are considered identical might still have a distance
    /// between them, due to inaccuracies of the numerical representation. If
    /// that distance is less than the one defined in this field, can not be
    /// considered identical.
    pub identical_max_distance: Scalar,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            distinct_min_distance: Scalar::from_f64(5e-7), // 0.5 µm,

            // This value was chosen pretty arbitrarily. Seems small enough to
            // catch errors. If it turns out it's too small (because it produces
            // false positives due to floating-point accuracy issues), we can
            // adjust it.
            identical_max_distance: Scalar::from_f64(5e-14),
        }
    }
}

/// An error that can occur during a validation
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// `Cycle` validation error
    #[error(transparent)]
    Cycle(#[from] CycleValidationError),

    /// `Face` validation error
    #[error(transparent)]
    Face(#[from] FaceValidationError),

    /// `HalfEdge` validation error
    #[error(transparent)]
    HalfEdge(#[from] HalfEdgeValidationError),

    /// `SurfaceVertex` position didn't match `GlobalVertex`
    #[error(transparent)]
    SurfaceVertex(#[from] SurfaceVertexValidationError),

    /// `Vertex` validation error
    #[error(transparent)]
    Vertex(#[from] VertexValidationError),
}

impl From<Infallible> for ValidationError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}
