//! Service API that promotes monitoring and interactivity
//!
//! See [`Layers`].

mod layer;
mod objects;
mod validation;

use crate::{
    objects::{AboutToBeStored, AnyObject, Objects},
    validate::{ValidationConfig, ValidationErrors},
};

pub use self::{
    layer::{Layer, State},
    objects::{InsertObject, Operation},
    validation::{Validation, ValidationCommand, ValidationEvent},
};

/// The kernel services
#[derive(Default)]
pub struct Layers {
    /// The objects service
    ///
    /// Allows for inserting objects into a store after they were created.
    pub objects: Layer<Objects>,

    /// The validation service
    ///
    /// Validates objects that are inserted using the objects service.
    pub validation: Layer<Validation>,
}

impl Layers {
    /// Construct an instance of `Services`
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an instance of `Services`, using the provided configuration
    pub fn with_validation_config(config: ValidationConfig) -> Self {
        let objects = Layer::default();
        let validation = Layer::new(Validation::with_validation_config(config));

        Self {
            objects,
            validation,
        }
    }

    /// Insert an object into the stores
    pub fn insert_object(&mut self, object: AnyObject<AboutToBeStored>) {
        let mut object_events = Vec::new();
        self.objects
            .process(Operation::InsertObject { object }, &mut object_events);

        for object_event in object_events {
            let command = ValidationCommand::ValidateObject {
                object: object_event.object.into(),
            };
            self.validation.process(command, &mut Vec::new());
        }
    }

    /// Drop `Services`; return any unhandled validation error
    pub fn drop_and_validate(self) -> Result<(), ValidationErrors> {
        let errors = self.validation.into_state().into_errors();

        if errors.0.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
