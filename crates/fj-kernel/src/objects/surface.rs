use fj_math::{Line, Point, Vector};

use super::CurveKind;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Surface {
    /// The curve that this surface was swept from
    pub curve: CurveKind<3>,

    /// The path that the curve was swept along
    pub path: Vector<3>,
}

impl Surface {
    /// Construct a `Surface` that represents the xy-plane
    pub fn xy_plane() -> Self {
        Self {
            curve: CurveKind::x_axis(),
            path: Vector::unit_y(),
        }
    }

    /// Construct a `Surface` that represents the xz-plane
    pub fn xz_plane() -> Self {
        Self {
            curve: CurveKind::x_axis(),
            path: Vector::unit_z(),
        }
    }

    /// Construct a `Surface` that represents the yz-plane
    pub fn yz_plane() -> Self {
        Self {
            curve: CurveKind::y_axis(),
            path: Vector::unit_z(),
        }
    }

    /// Construct a plane from 3 points
    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let curve = CurveKind::Line(Line::from_points([a, b]));
        let path = c - a;

        Self { curve, path }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        let point = point.into();
        self.curve.point_from_curve_coords([point.u])
            + self.path_to_line().vector_from_line_coords([point.v])
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        let vector = vector.into();
        self.curve.vector_from_curve_coords([vector.u])
            + self.path_to_line().vector_from_line_coords([vector.v])
    }

    fn path_to_line(&self) -> Line<3> {
        Line::from_origin_and_direction(self.curve.origin(), self.path)
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::objects::CurveKind;

    use super::Surface;

    #[test]
    fn point_from_surface_coords() {
        let swept = Surface {
            curve: CurveKind::Line(Line::from_origin_and_direction(
                Point::from([1., 1., 1.]),
                Vector::from([0., 2., 0.]),
            )),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.point_from_surface_coords([2., 4.]),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let swept = Surface {
            curve: CurveKind::Line(Line::from_origin_and_direction(
                Point::from([1., 0., 0.]),
                Vector::from([0., 2., 0.]),
            )),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.vector_from_surface_coords([2., 4.]),
            Vector::from([0., 4., 8.]),
        );
    }
}
