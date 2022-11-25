use std::collections::BTreeMap;

use crate::{
    objects::{BehindHandle, Object},
    storage::ObjectId,
    validate::ValidationError,
};

use super::{objects::ObjectToInsert, State};

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation(
    pub BTreeMap<ObjectId, (Object<BehindHandle>, ValidationError)>,
);

impl Drop for Validation {
    fn drop(&mut self) {
        if !self.0.is_empty() {
            println!("Dropping `Validation` with unhandled validation errors:");

            for (_, err) in self.0.values() {
                println!("{err}");
            }

            panic!();
        }
    }
}

impl State for Validation {
    type Command = ObjectToInsert;
    type Event = ValidationEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let err = command.object.validate().err();
        events.push(ValidationEvent {
            object: command.object.into(),
            err,
        });
    }

    fn evolve(&mut self, event: &Self::Event) {
        if let Some(err) = &event.err {
            self.0
                .insert(event.object.id(), (event.object.clone(), err.clone()));
        }
    }
}

/// An event produced by the validation service
#[derive(Clone)]
pub struct ValidationEvent {
    /// The object for which validation has been attempted
    pub object: Object<BehindHandle>,

    /// The validation error, if the validation resulted in one
    ///
    /// If this is `None`, the object has been validated successfully.
    pub err: Option<ValidationError>,
}
