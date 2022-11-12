//! The geometry that defines a surface

use fj_math::{Line, Point, Vector};

use super::path::GlobalPath;

/// The geometry that defines a surface
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceGeometry {
    /// The u-axis of the surface
    pub u: GlobalPath,

    /// The v-axis of the surface
    pub v: Vector<3>,
}

impl SurfaceGeometry {
    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        let point = point.into();
        self.u.point_from_path_coords([point.u])
            + self.path_to_line().vector_from_line_coords([point.v])
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        let vector = vector.into();
        self.u.vector_from_path_coords([vector.u])
            + self.path_to_line().vector_from_line_coords([vector.v])
    }

    fn path_to_line(&self) -> Line<3> {
        Line::from_origin_and_direction(self.u.origin(), self.v)
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::geometry::{path::GlobalPath, surface::SurfaceGeometry};

    #[test]
    fn point_from_surface_coords() {
        let surface = SurfaceGeometry {
            u: GlobalPath::Line(Line::from_origin_and_direction(
                Point::from([1., 1., 1.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            surface.point_from_surface_coords([2., 4.]),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let surface = SurfaceGeometry {
            u: GlobalPath::Line(Line::from_origin_and_direction(
                Point::from([1., 0., 0.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        assert_eq!(
            surface.vector_from_surface_coords([2., 4.]),
            Vector::from([0., 4., 8.]),
        );
    }
}
