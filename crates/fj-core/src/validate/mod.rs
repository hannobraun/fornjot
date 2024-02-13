//! # Infrastructure for validating objects
//!
//! ## Structure and Nomenclature
//!
//! **Validation** is the process of checking that objects meet specific
//! requirements. Each kind of object has its own set of requirements.
//!
//! An object that meets all the requirement for its kind is considered
//! **valid**. An object that does not meet all of them is considered
//! **invalid**. This results in a **validation error**, which is represented by
//! [`ValidationError`].
//!
//! Every single requirement is checked by a dedicated function. These functions
//! are called **validation checks**. Validation checks are currently not
//! visible in the public API, but their structure is reflected in the variants
//! of the different enums that make up [`ValidationError`] (each validation
//! check produces one of the types of validation error, that these nested enums
//! represent).
//!
//! In principle, the absence of validation errors should guarantee, that an
//! object can be exported to an external file format without problems (which
//! falls under the purview of the [`fj-export`] crate). This has not yet been
//! achieved, as some validation checks are still missing.
//!
//! The [issue tracker] has open issues for some of those missing checks, but
//! others are not currently tracked (or not even known). Please feel free to
//! open a new issue (or comment on an existing one), if you encounter an object
//! that you believe should be invalid, but is not.
//!
//!
//! ## Use
//!
//! All objects implement the [`Validate`] trait, which users can use to
//! validate objects manually. This might be useful for debugging, but is
//! otherwise not recommended.
//!
//! Experience has shown, that stopping the construction of a shape on the first
//! validation failure can make it hard to understand what actually went wrong.
//! For that reason, objects are validated as they are constructed, but
//! validation errors are collected in the background, to be processed when the
//! whole shape has been finished.
//!
//! This is set up within the [`Services`] API, and validation errors result in
//! a panic, when the [`Services`] instance is dropped. Unless you want to
//! handle validation errors in a different way, you don't have to do anything
//! special to use the validation infrastructure.
//!
//!
//! ## Configuration
//!
//! Fornjot's geometry representation is set up to prioritize correctness, which
//! is achieved by making the relations between different objects *explicit*.
//! This means, for example, that coincident objects of the same type that don't
//! have the same *identity* are generally considered invalid.
//!
//! Coincidence checks must use a tolerance value to be useful, meaning objects
//! that are very close together can be considered coincident. What should be
//! considered "very close" is dependent on the scale that your model operates
//! on, and this fact is taken into account by allowing for configuration via
//! [`Validate::validate_with_config`] and [`ValidationConfig`].
//!
//!
//! [`fj-export`]: https://crates.io/crates/fj-export
//! [issue tracker]: https://github.com/hannobraun/fornjot/issues
//! [`Services`]: crate::layers::Services

mod curve;
mod cycle;
mod edge;
mod face;
mod references;
mod region;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

pub use self::{
    cycle::CycleValidationError, edge::EdgeValidationError,
    face::FaceValidationError, shell::ShellValidationError,
    sketch::SketchValidationError, solid::SolidValidationError,
};

use std::{convert::Infallible, fmt};

use fj_math::Scalar;

/// Assert that some object has a validation error which matches a specific
/// pattern. This is preferred to matching on [`Validate::validate_and_return_first_error`], since usually we don't care about the order.
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
#[derive(Clone, Debug, thiserror::Error)]
pub enum ValidationError {
    /// `Cycle` validation error
    #[error("`Cycle` validation error")]
    Cycle(#[from] CycleValidationError),

    /// `Edge` validation error
    #[error("`Edge` validation error")]
    Edge(#[from] EdgeValidationError),

    /// `Face` validation error
    #[error("`Face` validation error")]
    Face(#[from] FaceValidationError),

    /// `Shell` validation error
    #[error("`Shell` validation error")]
    Shell(#[from] ShellValidationError),

    /// `Solid` validation error
    #[error("`Solid` validation error")]
    Solid(#[from] SolidValidationError),

    /// `Sketch` validation error
    #[error("`Sketch` validation error")]
    Sketch(#[from] SketchValidationError),
}

impl From<Infallible> for ValidationError {
    fn from(infallible: Infallible) -> Self {
        match infallible {}
    }
}

/// A collection of validation errors
#[derive(Debug, thiserror::Error)]
pub struct ValidationErrors(pub Vec<ValidationError>);

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_errors = self.0.len();

        writeln!(f, "{num_errors} unhandled validation errors:")?;

        for err in &self.0 {
            writeln!(f, "{err}")?;
        }

        Ok(())
    }
}
