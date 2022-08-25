//! Reverse the direction/orientation of objects

mod face;

/// Reverse the direction/orientation of an object
pub trait Reverse {
    /// Reverse the direction/orientation of the object
    #[must_use]
    fn reverse(self) -> Self;
}
