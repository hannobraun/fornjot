use std::{error::Error, thread};

use super::{ValidationConfig, ValidationError};

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation {
    /// All unhandled validation errors
    pub errors: Vec<ValidationError>,

    /// Validation configuration for the validation service
    pub config: ValidationConfig,
}

impl Validation {
    /// Construct an instance of `Validation`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let errors = Vec::new();
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

            for err in self.errors.iter() {
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
