use fj_math::{Line, Point, Transform, Vector};

use crate::geometry::Curve;

/// A surface that was swept from a curve
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SweptCurve {
    /// The curve that this surface was swept from
    pub curve: Curve,

    /// The path that the curve was swept along
    pub path: Vector<3>,
}

impl SweptCurve {
    /// Construct a plane from 3 points
    #[cfg(test)]
    pub fn plane_from_points([a, b, c]: [Point<3>; 3]) -> Self {
        let curve = Curve::Line(Line::from_points([a, b]));
        let path = c - a;

        Self { curve, path }
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.curve = self.curve.reverse();
        self
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        self.curve = self.curve.transform(transform);
        self.path = transform.transform_vector(&self.path);
        self
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn convert_point_to_surface_coords(
        &self,
        point: &Point<3>,
    ) -> Point<2> {
        let u = self.curve.point_to_curve_coords(point).t;
        let v = {
            let line = Line {
                origin: self.curve.origin(),
                direction: self.path,
            };

            line.point_to_line_coords(*point).t
        };

        Point::from([u, v])
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn convert_point_from_surface_coords(
        &self,
        point: &Point<2>,
    ) -> Point<3> {
        self.curve.point_from_curve_coords(&point.to_t()) + self.path * point.v
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn convert_vector_from_surface_coords(
        &self,
        vector: &Vector<2>,
    ) -> Vector<3> {
        self.curve.vector_from_curve_coords(&vector.to_t())
            + self.path * vector.v
    }
}

#[cfg(test)]
mod tests {

    use fj_math::{Line, Point, Vector};

    use crate::geometry::Curve;

    use super::SweptCurve;

    #[test]
    fn convert_point_to_surface_coords() {
        let swept = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 3.]),
        };

        verify(&swept, Point::from([-1., -1.]));
        verify(&swept, Point::from([0., 0.]));
        verify(&swept, Point::from([1., 1.]));
        verify(&swept, Point::from([2., 3.]));

        fn verify(swept: &SweptCurve, surface_point: Point<2>) {
            let point = swept.convert_point_from_surface_coords(&surface_point);
            let result = swept.convert_point_to_surface_coords(&point);

            assert_eq!(result, surface_point);
        }
    }

    #[test]
    fn convert_point_from_surface_coords() {
        let swept = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.convert_point_from_surface_coords(&Point::from([2., 4.])),
            Point::from([1., 4., 8.]),
        );
    }

    #[test]
    fn convert_vector_from_surface_coords() {
        let swept = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.convert_vector_from_surface_coords(&Vector::from([2., 4.])),
            Vector::from([0., 4., 8.]),
        );
    }
}
