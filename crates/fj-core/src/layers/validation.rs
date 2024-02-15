//! Layer infrastructure for [`Validation`]

use crate::{
    objects::{AnyObject, Stored},
    validate::{Validation, ValidationError, ValidationErrors},
};

use super::{objects::InsertObject, Command, Event, Layer};

impl Layer<Validation> {
    /// Take all errors stored in the validation layer
    pub fn take_errors(&mut self) -> Result<(), ValidationErrors> {
        self.process(TakeErrors, &mut Vec::new())
    }
}

impl Command<Validation> for InsertObject {
    type Result = ();
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

/// Take all errors stored in the validation layer
///
/// Serves both as a command for and event produced by `Layer<Validation>`.
pub struct TakeErrors;

impl Command<Validation> for TakeErrors {
    type Result = Result<(), ValidationErrors>;
    type Event = Self;

    fn decide(
        self,
        state: &Validation,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        let errors = ValidationErrors(state.errors.values().cloned().collect());

        events.push(self);

        if errors.0.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Event<Validation> for TakeErrors {
    fn evolve(&self, state: &mut Validation) {
        state.errors.clear();
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
