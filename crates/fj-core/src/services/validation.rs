use std::{collections::BTreeMap, error::Error, thread};

use crate::{
    objects::{AnyObject, Stored},
    storage::ObjectId,
    validate::{ValidationConfig, ValidationError},
};

use super::State;

/// Errors that occurred while validating the objects inserted into the stores
pub struct Validation {
    /// All unhandled validation errors
    pub errors: BTreeMap<ObjectId, ValidationError>,

    /// Validation configuration for the validation service
    config: ValidationConfig,
}

impl Validation {
    /// A constructor for the validation service that allows a validation configuration to be set for the service
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let errors = BTreeMap::new();
        Self { errors, config }
    }
}

impl Default for Validation {
    fn default() -> Self {
        let errors = BTreeMap::new();
        let config: ValidationConfig = ValidationConfig::default();
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

impl State for Validation {
    type Command = ValidationCommand;
    type Event = ValidationEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let mut errors = Vec::new();

        match command {
            ValidationCommand::ValidateObject { object } => {
                object.validate_with_config(&self.config, &mut errors);

                for err in errors {
                    events.push(ValidationEvent::ValidationFailed {
                        object: object.clone(),
                        err,
                    });
                }
            }
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        match event {
            ValidationEvent::ValidationFailed { object, err } => {
                self.errors.insert(object.id(), err.clone());
            }
        }
    }
}

/// The command accepted by the validation service
pub enum ValidationCommand {
    /// Validate the provided object
    ValidateObject {
        /// The object to validate
        object: AnyObject<Stored>,
    },
}

/// The event produced by the validation service
#[derive(Clone)]
pub enum ValidationEvent {
    /// Validation of an object failed
    ValidationFailed {
        /// The object for which validation failed
        object: AnyObject<Stored>,

        /// The validation error
        err: ValidationError,
    },
}
