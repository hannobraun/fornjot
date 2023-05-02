use std::{collections::BTreeMap, thread};

use crate::{
    objects::{BehindHandle, Object},
    storage::ObjectId,
    validate::ValidationError,
};

use super::State;

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation(pub BTreeMap<ObjectId, ValidationError>);

impl Drop for Validation {
    fn drop(&mut self) {
        let num_errors = self.0.len();
        if num_errors > 0 {
            println!(
                "Dropping `Validation` with {num_errors} unhandled validation \
                errors:"
            );

            for err in self.0.values() {
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
    type Event = ValidationFailed;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let ValidationCommand::ValidateObject { object } = command;

        let mut errors = Vec::new();
        object.validate(&mut errors);

        for err in errors {
            events.push(ValidationFailed {
                object: object.clone(),
                err,
            });
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        self.0.insert(event.object.id(), event.err.clone());
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
pub struct ValidationFailed {
    /// The object for which validation failed
    pub object: Object<BehindHandle>,

    /// The validation error
    pub err: ValidationError,
}
