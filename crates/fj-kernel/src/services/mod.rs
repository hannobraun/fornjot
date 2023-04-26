//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;
mod validation;

use crate::objects::{Object, Objects, WithHandle};

pub use self::{
    objects::{InsertObject, Operation},
    service::{Service, State},
    validation::{Validation, ValidationFailed},
};

/// The kernel services
pub struct Services {
    /// The objects service
    ///
    /// Allows for inserting objects into a store after they were created.
    pub objects: Service<Objects>,

    /// The validation service
    ///
    /// Validates objects that are inserted using the objects service.
    pub validation: Service<Validation>,
}

impl Services {
    /// Construct an instance of `Services`
    pub fn new() -> Self {
        let objects = Service::<Objects>::default();
        let validation = Service::default();

        Self {
            objects,
            validation,
        }
    }

    /// Insert an object into the stores
    pub fn insert_object(&mut self, object: Object<WithHandle>) {
        let mut object_events = Vec::new();
        self.objects
            .execute(Operation::InsertObject { object }, &mut object_events);

        for object_event in object_events {
            self.validation.execute(object_event, &mut Vec::new());
        }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
