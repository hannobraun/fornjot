use nalgebra::{Point2, RealField as _};

use crate::geometry::Circle;

use super::Boundary;

impl Boundary for Circle {
    fn boundary(&self, s: f32) -> Point2<f32> {
        let angle = f32::two_pi() * s;

        let (sin, cos) = angle.sin_cos();

        let x = cos * self.radius();
        let y = sin * self.radius();

        Point2::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use nalgebra::Point2;

    use crate::geometry::{Boundary as _, Circle};

    #[test]
    fn circle_should_return_boundary_points() {
        let circle = Circle::from_radius(1.0);

        assert_relative_eq!(circle.boundary(0.0), Point2::new(1.0, 0.0));
        assert_relative_eq!(circle.boundary(0.25), Point2::new(0.0, 1.0));
        assert_relative_eq!(circle.boundary(0.5), Point2::new(-1.0, 0.0));
        assert_relative_eq!(circle.boundary(0.75), Point2::new(0.0, -1.0));
        assert_relative_eq!(
            circle.boundary(1.0),
            Point2::new(1.0, 0.0),
            epsilon = 1e-6
        );
    }
}
