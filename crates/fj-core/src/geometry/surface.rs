//! The geometry that defines a surface

use fj_math::{Line, Plane, Point, Transform, Vector};

use super::GlobalPath;

/// The geometry that defines a surface
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SurfaceGeom {
    /// # Basic definition of surface geometry
    ///
    /// ## Implementation Note
    ///
    /// At the time of writing, this is the sole variant of `SurfaceGeom`.
    /// `SurfaceGeom` simply used to be a struct, identical to this variant.
    ///
    /// This was changed as part of a transition to a new, less basic and more
    /// flexible, representation of surface geometry.
    Basic {
        /// The u-axis of the surface
        u: GlobalPath,

        /// The v-axis of the surface
        v: Vector<3>,
    },
}

impl SurfaceGeom {
    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
    ) -> Point<3> {
        let point = point.into();
        let Self::Basic { u, .. } = self;
        u.point_from_path_coords([point.u])
            + self.path_to_line().vector_from_line_coords([point.v])
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
    ) -> Vector<3> {
        let vector = vector.into();
        let Self::Basic { u, .. } = self;
        u.vector_from_path_coords([vector.u])
            + self.path_to_line().vector_from_line_coords([vector.v])
    }

    fn path_to_line(&self) -> Line<3> {
        let Self::Basic { u, v } = self;
        Line::from_origin_and_direction(u.origin(), *v)
    }

    /// Project the global point into the surface
    pub fn project_global_point(&self, point: impl Into<Point<3>>) -> Point<2> {
        let Self::Basic { u, v } = self;

        let GlobalPath::Line(line) = u else {
            todo!("Projecting point into non-plane surface is not supported")
        };

        let plane = Plane::from_parametric(line.origin(), line.direction(), *v);
        plane.project_point(point)
    }

    /// Transform the surface geometry
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        let Self::Basic { u, v } = self;

        let u = u.transform(transform);
        let v = transform.transform_vector(&v);
        Self::Basic { u, v }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::geometry::{GlobalPath, SurfaceGeom};

    #[test]
    fn point_from_surface_coords() {
        let surface = SurfaceGeom::Basic {
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
        let surface = SurfaceGeom::Basic {
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
