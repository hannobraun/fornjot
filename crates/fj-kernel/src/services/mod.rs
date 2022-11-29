//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;

use crate::objects::Objects;

pub use self::{
    objects::ServiceObjectsExt,
    service::{Service, State},
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
}

impl Services {
    /// Construct an instance of `Services`
    pub fn new() -> Self {
        let objects = Service::default();
        Self { objects }
    }
}

impl Default for Services {
    fn default() -> Self {
        Self::new()
    }
}
