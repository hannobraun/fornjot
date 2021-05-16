use nalgebra::Point;

/// Provides a signed distance function
pub trait Distance<const D: usize> {
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32;
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::Cylinder;

    use super::Distance as _;

    #[test]
    fn cylinder_should_return_distance() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        assert_eq!(cylinder.distance([0.0, 0.0, 0.0]), -0.5);
        assert_eq!(cylinder.distance([0.25, 0.0, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.25, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.0, 0.25]), -0.25);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -1.0]), 0.5);
        assert_eq!(cylinder.distance([1.0, 0.0, 2.0]), 0.5);
    }
}
