//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;
mod validation;

use std::sync::Arc;

use parking_lot::Mutex;

use crate::objects::Objects;

pub use self::{
    objects::ServiceObjectsExt,
    service::{Service, State},
    validation::{Validation, ValidationEvent},
};

/// The kernel services
pub struct Services {
    /// The objects service
    ///
    /// Allows for inserting objects into a store after they were created.
    ///
    /// [`ServiceObjectsExt`] is available to provide a convenient API around
    /// this service.
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
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
