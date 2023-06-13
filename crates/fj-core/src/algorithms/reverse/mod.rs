//! Reverse the direction/orientation of objects

use crate::services::Services;

mod cycle;
mod face;

/// Reverse the direction/orientation of an object
pub trait Reverse: Sized {
    /// Reverse the direction/orientation of the object
    fn reverse(&self, services: &mut Services) -> Self;
}
