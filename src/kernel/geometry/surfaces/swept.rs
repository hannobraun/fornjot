use nalgebra::{point, vector};
use parry3d_f64::math::Isometry;

use crate::{
    kernel::geometry::Curve,
    math::{Point, Vector},
};

/// A surface that was swept from a curve
#[derive(Clone, Debug, PartialEq)]
pub struct Swept {
    /// The curve that this surface was swept from
    pub curve: Curve,

    /// The path that the curve was swept along
    pub path: Vector<3>,
}

impl Swept {
    /// Transform the surface
    #[must_use]
    pub fn transform(mut self, transform: &Isometry<f64>) -> Self {
        self.curve = self.curve.transform(transform);
        self.path = transform.transform_vector(&self.path);
        self
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point: &Point<3>) -> Point<2> {
        let p = point - self.curve.origin();

        let u = self.curve.point_model_to_curve(point).x;
        let v = p.dot(&self.path.normalize()) / self.path.magnitude();

        point![u, v]
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: &Point<2>) -> Point<3> {
        let u = point.x;
        let v = point.y;

        self.curve.point_curve_to_model(&point![u]) + self.path * v
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: &Vector<2>) -> Vector<3> {
        let u = vector.x;
        let v = vector.y;

        self.curve.vector_curve_to_model(&vector![u]) + self.path * v
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use crate::{
        kernel::geometry::{Curve, Line},
        math::Point,
    };

    use super::Swept;

    #[test]
    fn point_model_to_surface() {
        let swept = Swept {
            curve: Curve::Line(Line {
                origin: point![1., 0., 0.],
                direction: vector![0., 2., 0.],
            }),
            path: vector![0., 0., 2.],
        };

        verify(&swept, point![-1., -1.]);
        verify(&swept, point![0., 0.]);
        verify(&swept, point![1., 1.]);
        verify(&swept, point![2., 3.]);

        fn verify(swept: &Swept, surface_point: Point<2>) {
            let point = swept.point_surface_to_model(&surface_point);
            let result = swept.point_model_to_surface(&point);

            assert_eq!(result, surface_point);
        }
    }

    #[test]
    fn point_surface_to_model() {
        let swept = Swept {
            curve: Curve::Line(Line {
                origin: point![1., 0., 0.],
                direction: vector![0., 2., 0.],
            }),
            path: vector![0., 0., 2.],
        };

        assert_eq!(
            swept.point_surface_to_model(&point![2., 4.]),
            point![1., 4., 8.],
        );
    }

    #[test]
    fn vector_surface_to_model() {
        let swept = Swept {
            curve: Curve::Line(Line {
                origin: point![1., 0., 0.],
                direction: vector![0., 2., 0.],
            }),
            path: vector![0., 0., 2.],
        };

        assert_eq!(
            swept.vector_surface_to_model(&vector![2., 4.]),
            vector![0., 4., 8.],
        );
    }
}
