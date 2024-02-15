//! Layer infrastructure for [`Validation`]

use crate::{
    objects::{AnyObject, Stored},
    validate::{Validation, ValidationError, ValidationErrors},
};

use super::{objects::InsertObject, Event, Layer, State};

impl Layer<Validation> {
    /// Handler for [`InsertObject`]
    pub fn on_objects_event(&mut self, event: InsertObject) {
        let command = ValidationCommand::ValidateObject {
            object: event.object.into(),
        };
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

impl State for Validation {
    type Command = ValidationCommand;
    type Event = ValidationFailed;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let mut errors = Vec::new();

        match command {
            ValidationCommand::ValidateObject { object } => {
                object.validate_with_config(&self.config, &mut errors);

                for err in errors {
                    events.push(ValidationFailed {
                        object: object.clone(),
                        err,
                    });
                }
            }
        }
    }
}

/// Command for `Layer<Validation>`
pub enum ValidationCommand {
    /// Validate the provided object
    ValidateObject {
        /// The object to validate
        object: AnyObject<Stored>,
    },
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
