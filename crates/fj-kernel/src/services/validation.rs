use std::{collections::BTreeMap, thread};

use crate::{
    objects::{BehindHandle, Object},
    storage::ObjectId,
    validate::ValidationError,
};

use super::{objects::ObjectToInsert, State};

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation(pub BTreeMap<ObjectId, ValidationFailed>);

impl Drop for Validation {
    fn drop(&mut self) {
        let num_errors = self.0.len();
        if num_errors > 0 {
            println!(
                "Dropping `Validation` with {num_errors} unhandled validation \
                errors:"
            );

            for event in self.0.values() {
                println!("{}", event.err);
            }

            if !thread::panicking() {
                panic!();
            }
        }
    }
}

impl State for Validation {
    type Command = ObjectToInsert;
    type Event = ValidationFailed;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        if let Err(err) = command.object.validate() {
            events.push(ValidationFailed {
                object: command.object.into(),
                err,
            });
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        self.0.insert(event.object.id(), event.clone());
    }
}

/// An event produced by the validation service
#[derive(Clone)]
pub struct ValidationFailed {
    /// The object for which validation failed
    pub object: Object<BehindHandle>,

    /// The validation error
    pub err: ValidationError,
}
