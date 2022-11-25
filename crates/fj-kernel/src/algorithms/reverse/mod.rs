//! Reverse the direction/orientation of objects

use crate::{objects::Objects, services::Service, validate::ValidationError};

mod cycle;
mod edge;
mod face;

/// Reverse the direction/orientation of an object
pub trait Reverse: Sized {
    /// Reverse the direction/orientation of the object
    fn reverse(
        self,
        objects: &mut Service<Objects>,
    ) -> Result<Self, ValidationError>;
}
