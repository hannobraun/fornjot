use crate::geometry::operations::{Scale, Sweep};

use super::Circle;

/// A cylinder
///
/// Defined as a [`Sweep`] of a [`Circle`].
pub type Cylinder = Sweep<Scale<Circle>, f32>;

impl Cylinder {
    /// Create a new `Cylinder`
    ///
    /// Sweeps a default [`Circle`] along a distance of `1.0`.
    pub fn new() -> Self {
        Sweep {
            shape: Circle::from_radius(1.0),
            path: 1.0,
        }
    }

    /// Update radius
    ///
    /// Returns a copy of `self` with the radius replaced with `radius`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.shape = self.shape.with_factor(radius);
        self
    }

    /// Update height
    ///
    /// Returns a copy of `self` with the height replaced with `height`.
    pub fn with_height(self, height: f32) -> Self {
        self.with_path(height)
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::attributes::SignedDistanceField as _;

    use super::Cylinder;

    #[test]
    fn distance() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        assert_eq!(cylinder.distance([0.0, 0.0, 0.5]).distance, -0.5);
        assert_eq!(cylinder.distance([0.25, 0.0, 0.5]).distance, -0.25);
        assert_eq!(cylinder.distance([0.0, 0.25, 0.5]).distance, -0.25);
        assert_eq!(cylinder.distance([0.0, 0.0, 0.75]).distance, -0.25);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.5]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -0.5]).distance, 0.5);
    }
}
