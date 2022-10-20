//! Reverse the direction/orientation of objects

use crate::objects::Objects;

mod cycle;
mod edge;
mod face;

/// Reverse the direction/orientation of an object
pub trait Reverse {
    /// Reverse the direction/orientation of the object
    #[must_use]
    fn reverse(self, objects: &Objects) -> Self;
}
