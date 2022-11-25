//! Service API that promotes monitoring and interactivity
//!
//! See [`Service`].

mod objects;
mod service;

pub use self::{
    objects::ServiceObjectsExt,
    service::{Service, State},
};
