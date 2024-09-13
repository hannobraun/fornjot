//! The geometry that defines a surface

use fj_math::{Point, Scalar, Transform, Triangle, Vector};

use super::{GenPolyline, Path, Tolerance};

/// The geometry that defines a surface
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceGeom {
    /// The u-axis of the surface
    pub u: Path<3>,

    /// The v-axis of the surface
    pub v: Vector<3>,
}

impl SurfaceGeom {
    /// # Access the origin of the surface
    pub fn origin(&self) -> Point<3> {
        self.u.origin()
    }

    /// # Return the triangle at the provided point on the surface
    ///
    /// Select a triangle of the surface's triangle mesh representation, the one
    /// at the provided surface point. Return that triangle, as well as the
    /// barycentric coordinates of the provided point on the triangle.
    ///
    /// ## Triangle Size and Validity
    ///
    /// If a surface is curved along both axes, the triangle's size is chosen
    /// such, that it approximates the surface, with the maximum allowed
    /// deviation of the actual surface defined by the provided tolerance
    /// argument.
    ///
    /// Otherwise, the size of the returned triangle is at least partially
    /// arbitrary. Take the extreme case of a plane: Since it is not curved at
    /// all, the returned triangle can be arbitrarily large.
    ///
    /// However, since surfaces are infinite, and we can't represent infinite
    /// triangles, there is no sensible upper bound for the size. Instead, to
    /// prevent an arbitrary choice for the size of triangles, which would imply
    /// properties of the surface that are not true, and might therefore be
    /// confusing, the triangles returned by this function have a length of zero
    /// along axes that do not require approximation.
    ///
    /// The most extreme case would be a plane, for which the returned triangle
    /// is collapsed to a point. For a cylinder, the triangle would have the
    /// appropriate width to approximate the curved axis given the provided
    /// tolerance, while having zero height.
    ///
    /// ## Implementation Note
    ///
    /// At the time this was written, there was no dedicated type to represent
    /// barycentric coordinates. Nor any other code that used them, I think.
    ///
    /// If this changes, and a special type for barycentric coordinates is
    /// added, it would make sense to return that here.
    pub fn triangle_at(
        &self,
        point_surface: impl Into<Point<2>>,
        tolerance: impl Into<Tolerance>,
    ) -> (Triangle<3>, [Scalar; 3]) {
        let point_surface = point_surface.into();

        let [a, b] = self
            .u
            .line_segment_at([point_surface.u], tolerance)
            .map(|point_global| point_global + self.v * point_surface.v);

        let c = a + (b - a) / 2.;
        let triangle = Triangle::from([a, b, c]);

        let barycentric_coords = [1. / 3.; 3].map(Into::into);
        (triangle, barycentric_coords)
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_from_surface_coords(
        &self,
        point: impl Into<Point<2>>,
        tolerance: impl Into<Tolerance>,
    ) -> Point<3> {
        let (triangle, barycentric_coords) = self.triangle_at(point, tolerance);
        triangle.point_from_barycentric_coords(barycentric_coords)
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_from_surface_coords(
        &self,
        vector: impl Into<Vector<2>>,
        tolerance: impl Into<Tolerance>,
    ) -> Vector<3> {
        let vector = vector.into();
        let point =
            self.point_from_surface_coords(Point { coords: vector }, tolerance);
        point - self.origin()
    }

    /// Transform the surface geometry
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        let Self { u, v } = self;

        let u = u.transform(transform);
        let v = transform.transform_vector(&v);
        Self { u, v }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Vector};
    use pretty_assertions::assert_eq;

    use crate::geometry::{Path, SurfaceGeom, Tolerance};

    #[test]
    fn point_from_surface_coords() {
        let surface = SurfaceGeom {
            u: Path::Line(Line::from_origin_and_direction(
                Point::from([1., 1., 1.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        // Value doesn't matter; we're dealing with a plane.
        let tolerance = Tolerance::from_scalar(1.).unwrap();

        assert_eq!(
            surface.point_from_surface_coords([2., 4.], tolerance),
            Point::from([1., 5., 9.]),
        );
    }

    #[test]
    fn vector_from_surface_coords() {
        let surface = SurfaceGeom {
            u: Path::Line(Line::from_origin_and_direction(
                Point::from([1., 0., 0.]),
                Vector::from([0., 2., 0.]),
            )),
            v: Vector::from([0., 0., 2.]),
        };

        // Value doesn't matter; we're dealing with a plane.
        let tolerance = Tolerance::from_scalar(1.).unwrap();

        assert_eq!(
            surface.vector_from_surface_coords([2., 4.], tolerance),
            Vector::from([0., 4., 8.]),
        );
    }
}
