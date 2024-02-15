//! Layer infrastructure for [`Validation`]

use crate::{
    objects::{AnyObject, Stored},
    validate::{Validation, ValidationError, ValidationErrors},
};

use super::{objects::InsertObject, Command, Event, Layer};

impl Layer<Validation> {
    /// Handler for [`InsertObject`]
    pub fn on_insert_object(&mut self, event: InsertObject) {
        let command = event;
        self.process(command, &mut Vec::new());
    }

    /// Consume the validation layer, returning any validation errors
    pub fn into_result(self) -> Result<(), ValidationErrors> {
        let errors = self.into_state().into_errors();

        if errors.0.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Command<Validation> for InsertObject {
    type Event = ValidationFailed;

    fn decide(self, state: &Validation, events: &mut Vec<Self::Event>) {
        let mut errors = Vec::new();

        let object: AnyObject<Stored> = self.object.into();
        object.validate_with_config(&state.config, &mut errors);

        for err in errors {
            events.push(ValidationFailed {
                object: object.clone(),
                err,
            });
        }
    }
}

/// Validation of an object failed
///
/// Event produced by `Layer<Validation>`.
#[derive(Clone)]
pub struct ValidationFailed {
    /// The object for which validation failed
    pub object: AnyObject<Stored>,

    /// The validation error
    pub err: ValidationError,
}

impl Event<Validation> for ValidationFailed {
    fn evolve(&self, state: &mut Validation) {
        state.errors.insert(self.object.id(), self.err.clone());
    }
}
