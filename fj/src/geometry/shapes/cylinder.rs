use crate::geometry::operations::LinearSweep;

use super::Circle;

/// A cylinder
///
/// Defined as a [`Sweep`] of a [`Circle`].
pub type Cylinder = LinearSweep<Circle>;

impl Cylinder {
    /// Create a new `Cylinder`
    ///
    /// Sweeps a default [`Circle`] along a distance of `1.0`.
    pub fn new() -> Self {
        LinearSweep {
            sketch: Circle::new(),
            distance: 1.0,
        }
    }

    /// Update radius
    ///
    /// Returns a copy of `self` with the radius replaced with `radius`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.sketch = self.sketch.with_radius(radius);
        self
    }

    // `with_height` method is not required here, as `Sweep` already has it.
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
        let cylinder = Cylinder::new().with_radius(0.5).with_distance(1.0);

        assert_eq!(cylinder.distance([0.0, 0.0, 0.0]).distance, -0.5);
        assert_eq!(cylinder.distance([0.25, 0.0, 0.0]).distance, -0.25);
        assert_eq!(cylinder.distance([0.0, 0.25, 0.0]).distance, -0.25);
        assert_eq!(cylinder.distance([0.0, 0.0, 0.25]).distance, -0.25);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.0]).distance, 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -1.0]).distance, 0.5);
    }
}
