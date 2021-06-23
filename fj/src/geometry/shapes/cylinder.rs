use crate::geometry::operations::LinearExtrude;

use super::Circle;

pub type Cylinder = LinearExtrude<Circle>;

impl Cylinder {
    pub fn new() -> Self {
        LinearExtrude {
            sketch: Circle::new(),
            height: 1.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.sketch = self.sketch.with_radius(radius);
        self
    }

    // `with_height` method is not required here, as `LinearExtrude` already has
    // it.
}

#[cfg(test)]
mod tests {
    use nalgebra::{vector, Unit};

    use crate::geometry::attributes::Surface as _;

    use super::Cylinder;

    #[test]
    fn distance() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        assert_eq!(cylinder.sample([0.0, 0.0, 0.0]).distance, -0.5);
        assert_eq!(cylinder.sample([0.25, 0.0, 0.0]).distance, -0.25);
        assert_eq!(cylinder.sample([0.0, 0.25, 0.0]).distance, -0.25);
        assert_eq!(cylinder.sample([0.0, 0.0, 0.25]).distance, -0.25);

        assert_eq!(cylinder.sample([1.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.sample([0.0, 1.0, 0.0]).distance, 0.5);
        assert_eq!(cylinder.sample([0.0, 0.0, 1.0]).distance, 0.5);
        assert_eq!(cylinder.sample([0.0, 0.0, -1.0]).distance, 0.5);
    }

    #[test]
    fn normal() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        // The normal at the center is not defined. Just make sure we're not
        // panicking due to a divide by zero or something.
        let _ = cylinder.sample([0.0, 0.0, 0.0]);

        // Points that are above, below, or next to the cylinder. The resulting
        // normal will either come from the circle, or point directly up or
        // down.
        assert_eq!(
            cylinder.sample([2.0, 0.0, 0.0]).normal.into_inner(),
            vector![1.0, 0.0, 0.0],
        );
        assert_eq!(
            cylinder.sample([0.0, 2.0, 0.0]).normal.into_inner(),
            vector![0.0, 1.0, 0.0],
        );
        assert_eq!(
            cylinder.sample([0.0, 0.0, 2.0]).normal.into_inner(),
            vector![0.0, 0.0, 1.0],
        );
        assert_eq!(
            cylinder.sample([0.0, 0.0, -2.0]).normal.into_inner(),
            vector![0.0, 0.0, -1.0],
        );

        // Points that don't fulfill the conditions outlined above.
        assert_eq!(
            cylinder.sample([1.0, 0.0, 2.0]).normal,
            Unit::new_normalize(vector![1.0, 0.0, 1.0]),
        );
        assert_eq!(
            cylinder.sample([2.0, 0.0, -1.0]).normal,
            Unit::new_normalize(vector![1.0, 0.0, -1.0]),
        );
    }
}
