use std::{collections::BTreeMap, thread};

use crate::{
    objects::{BehindHandle, Object},
    storage::ObjectId,
    validate::ValidationError,
};

use super::State;

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation {
    errors: BTreeMap<ObjectId, ValidationError>,
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
        let ValidationCommand::ValidateObject { object } = command;

        let mut errors = Vec::new();
        object.validate(&mut errors);

        for err in errors {
            events.push(ValidationEvent::ValidationFailed {
                object: object.clone(),
                err,
            });
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        match event {
            ValidationEvent::ValidationFailed { object, err } => {
                self.errors.insert(object.id(), err.clone());
            }
            ValidationEvent::ClearErrors => self.errors.clear(),
        }
    }
}

/// The command accepted by the validation service
pub enum ValidationCommand {
    /// Validate the provided object
    ValidateObject {
        /// The object to validate
        object: Object<BehindHandle>,
    },
}

/// The event produced by the validation service
#[derive(Clone)]
pub enum ValidationEvent {
    /// Validation of an object failed
    ValidationFailed {
        /// The object for which validation failed
        object: Object<BehindHandle>,

        /// The validation error
        err: ValidationError,
    },

    /// All stored validation errors are being cleared
    ClearErrors,
}
