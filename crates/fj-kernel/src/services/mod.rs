//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;
mod validation;

use std::sync::Arc;

use parking_lot::Mutex;

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
    pub validation: Arc<Mutex<Service<Validation>>>,
}

impl Services {
    /// Construct an instance of `Services`
    pub fn new() -> Self {
        let mut objects = Service::<Objects>::default();
        let validation = Arc::new(Mutex::new(Service::default()));

        objects.subscribe(validation.clone());

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
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
