use crate::{
    kernel::geometry::Curve,
    math::{Point, Transform, Vector},
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
    pub fn transform(mut self, transform: &Transform) -> Self {
        self.curve = self.curve.transform(transform);
        self.path = transform.transform_vector(&self.path);
        self
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point: &Point<3>) -> Point<2> {
        let u = self.curve.point_model_to_curve(point).t();
        let v = (point - self.curve.origin()).dot(&self.path.normalize())
            / self.path.magnitude();

        Point::from([u, v])
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: &Point<2>) -> Point<3> {
        self.curve.point_curve_to_model(&Point::from([point.u()]))
            + self.path * point.v()
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: &Vector<2>) -> Vector<3> {
        self.curve
            .vector_curve_to_model(&Vector::from([vector.u()]))
            + self.path * vector.v()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        kernel::geometry::{Curve, Line},
        math::{Point, Vector},
    };

    use super::Swept;

    #[test]
    fn point_model_to_surface() {
        let swept = Swept {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        verify(&swept, Point::from([-1., -1.]));
        verify(&swept, Point::from([0., 0.]));
        verify(&swept, Point::from([1., 1.]));
        verify(&swept, Point::from([2., 3.]));

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
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.point_surface_to_model(&Point::from([2., 4.])),
            Point::from([1., 4., 8.]),
        );
    }

    #[test]
    fn vector_surface_to_model() {
        let swept = Swept {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.vector_surface_to_model(&Vector::from([2., 4.])),
            Vector::from([0., 4., 8.]),
        );
    }
}
