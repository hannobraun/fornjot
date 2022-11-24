//! Reverse the direction/orientation of objects

use crate::{objects::Objects, validate::ValidationError};

mod cycle;
mod edge;
mod face;

/// Reverse the direction/orientation of an object
pub trait Reverse: Sized {
    /// Reverse the direction/orientation of the object
    fn reverse(self, objects: &mut Objects) -> Result<Self, ValidationError>;
}
