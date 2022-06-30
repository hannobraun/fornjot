use fj_math::{Line, Point, Transform, Vector};

use crate::geometry;

use super::Curve;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Surface {
    /// A swept curve
    SweptCurve(SweptCurve),
}

impl Surface {
    /// Construct a `Surface` that represents the xy-plane
    pub fn xy_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::x_axis(),
            path: Vector::unit_y(),
        })
    }

    /// Construct a `Surface` that represents the xz-plane
    pub fn xz_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::x_axis(),
            path: Vector::unit_z(),
        })
    }

    /// Construct a `Surface` that represents the yz-plane
    pub fn yz_plane() -> Self {
        Self::SweptCurve(SweptCurve {
            curve: Curve::y_axis(),
            path: Vector::unit_z(),
        })
    }

    /// Construct a plane from 3 points
    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let curve = Curve::Line(Line::from_points([a, b]));
        let path = c - a;

        Self::SweptCurve(SweptCurve { curve, path })
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Self::SweptCurve(surface) => Self::SweptCurve(surface.reverse()),
        }
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::SweptCurve(surface) => {
                Self::SweptCurve(surface.transform(transform))
            }
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_to_surface_coords(
        &self,
        point_3d: impl Into<Point<3>>,
    ) -> geometry::LocalPoint<2> {
        let point_3d = point_3d.into();

        let point_2d = match self {
            Self::SweptCurve(surface) => {
                surface.point_to_surface_coords(point_3d)
            }
        };

        geometry::LocalPoint::new(point_2d, point_3d)
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        match self {
            Self::SweptCurve(surface) => {
                surface.point_from_surface_coords(point)
            }
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        match self {
            Self::SweptCurve(surface) => {
                surface.vector_from_surface_coords(vector)
            }
        }
    }
}

/// A surface that was swept from a curve
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SweptCurve {
    /// The curve that this surface was swept from
    pub curve: Curve<3>,

    /// The path that the curve was swept along
    pub path: Vector<3>,
}

impl SweptCurve {
    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.path = -self.path;
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
    pub fn point_to_surface_coords(
        &self,
        point: impl Into<Point<3>>,
    ) -> Point<2> {
        let point = point.into();

        let u = self.curve.point_to_curve_coords(point).t;
        let v = self.path_to_line().point_to_line_coords(point).t;

        Point::from([u, v])
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
        Line {
            origin: self.curve.origin(),
            direction: self.path,
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::objects::Curve;

    use super::SweptCurve;

    #[test]
    fn reverse() {
        let original = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 3.]),
        };

        let reversed = original.reverse();

        let expected = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., -3.]),
        };
        assert_eq!(expected, reversed);
    }

    #[test]
    fn point_to_surface_coords() {
        let plane = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 3.]),
        };

        verify(&plane, Point::from([-1., -1.]));
        verify(&plane, Point::from([0., 0.]));
        verify(&plane, Point::from([1., 1.]));
        verify(&plane, Point::from([2., 3.]));

        fn verify(swept: &SweptCurve, surface_point: Point<2>) {
            let point = swept.point_from_surface_coords(surface_point);
            let result = swept.point_to_surface_coords(point);

            assert_eq!(result, surface_point);
        }
    }

    #[test]
    fn point_from_surface_coords() {
        let swept = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 1., 1.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.point_from_surface_coords([2., 4.]),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let swept = SweptCurve {
            curve: Curve::Line(Line {
                origin: Point::from([1., 0., 0.]),
                direction: Vector::from([0., 2., 0.]),
            }),
            path: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            swept.vector_from_surface_coords([2., 4.]),
            Vector::from([0., 4., 8.]),
        );
    }
}
