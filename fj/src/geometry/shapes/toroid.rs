use crate::geometry::{operations::Sweep, shapes::Circle};

/// A toroid
///
/// Defined as a sweep of a shape around a circle.
pub type Toroid<T> = Sweep<T, Circle>;

impl<T> Toroid<T> {
    /// Create a new `Toroid` from a shape
    ///
    /// The radius of the circle is initially set to `1.0`.
    pub fn from_shape(shape: T) -> Self {
        Self {
            shape,
            path: Circle::new(),
        }
    }

    /// Update radius
    ///
    /// Returns a copy of `self`, with the radius replaced with `radius`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.path = self.path.with_radius(radius);
        self
    }
}
