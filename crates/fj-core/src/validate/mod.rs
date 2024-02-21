//! # Infrastructure for validating objects
//!
//! ## Structure
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
//! This is set up within the [`Layers`] API, and validation errors result in a
//! panic, when the [`Layers`] instance is dropped. Unless you want to handle
//! validation errors in a different way, you don't have to do anything special
//! to use the validation infrastructure.
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
//! ## Implementation Note
//!
//! This module is in the process of being replaced. See [`crate::validation`].
//!
//!
//! [`fj-export`]: https://crates.io/crates/fj-export
//! [issue tracker]: https://github.com/hannobraun/fornjot/issues
//! [`Layers`]: crate::layers::Layers

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

use crate::{storage::ObjectId, validation::ValidationError};

pub use self::{
    cycle::CycleValidationError, edge::EdgeValidationError,
    face::FaceValidationError, shell::ShellValidationError,
    sketch::SketchValidationError, solid::SolidValidationError,
};

use std::{collections::HashMap, error::Error, thread};

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

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation {
    /// All unhandled validation errors
    pub errors: HashMap<ObjectId, ValidationError>,

    /// Validation configuration for the validation service
    pub config: ValidationConfig,
}

impl Validation {
    /// Construct an instance of `Validation`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let errors = HashMap::new();
        Self { errors, config }
    }
}

impl Drop for Validation {
    fn drop(&mut self) {
        let num_errors = self.errors.len();
        if num_errors > 0 {
            println!(
                "Dropping `Validation` with {num_errors} unhandled validation \
                errors:"
            );

            for err in self.errors.values() {
                println!("{}", err);

                // Once `Report` is stable, we can replace this:
                // https://doc.rust-lang.org/std/error/struct.Report.html
                let mut source = err.source();
                while let Some(err) = source {
                    println!("\nCaused by:\n\t{err}");
                    source = err.source();
                }

                print!("\n\n");
            }

            if !thread::panicking() {
                panic!();
            }
        }
    }
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
