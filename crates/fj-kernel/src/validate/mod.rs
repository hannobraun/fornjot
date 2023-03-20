//! Infrastructure for validating objects

mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

use self::{cycle::CycleValidationError, shell::ShellValidationError};
pub use self::{edge::HalfEdgeValidationError, face::FaceValidationError};

use std::convert::Infallible;

use fj_math::Scalar;

/// Assert that some object has a validation error which matches a specifc pattern.
/// This is preferred to matching on [`Validate::validate_and_return_first_error`], since usually we don't care about the order.
#[macro_export]
macro_rules! assert_contains_err {
    ($o:tt,$p:pat) => {
        assert!({
            let mut errors = Vec::new();
            $o.validate(&mut errors);
            errors.iter().any(|e| matches!(e, $p))
        })
    };
}

/// Validate an object
///
/// This trait is used automatically when inserting an object into a store.
pub trait Validate: Sized {
    /// Validate the object using default config and return on first error
    #[allow(clippy::result_large_err)]
    fn validate_and_return_first_error(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();
        self.validate(&mut errors);

        if let Some(err) = errors.into_iter().next() {
            return Err(err);
        }

        Ok(())
    }

    /// Validate the object using default configuration
    fn validate(&self, errors: &mut Vec<ValidationError>) {
        self.validate_with_config(&ValidationConfig::default(), errors)
    }

    /// Validate the object
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    );
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

    /// How often to sample edges when checking if they coincide. This
    /// represents the number of points we check on each Edge.
    /// The higher this is the more precise our validation is, and the slower it is.
    pub sample_count: usize,
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

            // This value is completely arbitrary, but seems good enough for now.
            sample_count: 16,
        }
    }
}

/// An error that can occur during a validation
#[derive(Clone, Debug, thiserror::Error)]
pub enum ValidationError {
    /// `Face` validation error
    #[error("`Face` validation error:\n{0}")]
    Face(#[from] FaceValidationError),

    /// `HalfEdge` validation error
    #[error("`HalfEdge` validation error:\n{0}")]
    HalfEdge(#[from] HalfEdgeValidationError),

    /// `Cycle` validation error
    #[error("`Cycle` validation error:\n{0}")]
    Cycle(#[from] CycleValidationError),

    /// `Shell` validation error
    #[error("`Shell` validation error:\n{0}")]
    Shell(#[from] ShellValidationError),
}

impl From<Infallible> for ValidationError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}
