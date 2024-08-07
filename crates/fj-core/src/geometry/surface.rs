//! The geometry that defines a surface

use fj_math::{Line, Point, Scalar, Transform, Triangle, Vector};

use crate::algorithms::approx::{PathApproxParams, Tolerance};

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

        let Self::Basic { u, v } = self;
        match u {
            GlobalPath::Circle(circle) => {
                let params = PathApproxParams::for_circle(circle, tolerance);

                let a = point_surface.u - params.increment();
                let b = point_surface.u + params.increment();
                let c = a; // triangle is degenerate, as per function docs

                let triangle_points_in_circle_space = [a, b, c];
                let triangle_points_in_global_space =
                    triangle_points_in_circle_space
                        .map(|point_circle| {
                            circle.point_from_circle_coords([point_circle])
                        })
                        .map(|point_global| {
                            point_global + *v * point_surface.v
                        });

                let triangle = Triangle::from(triangle_points_in_global_space);
                let barycentric_coords = [0.5, 0.5, 0.0].map(Into::into);

                (triangle, barycentric_coords)
            }
            GlobalPath::Line(line) => {
                let a = line.direction();
                let b = *v;

                let point_global =
                    line.origin() + a * point_surface.u + b * point_surface.v;

                // We don't need to approximate a plane, so our triangle can be
                // arbitrarily large or small. Here we choose the smallest
                // possible size (it is collapsed to a point), as per the
                // documentation of this function.
                let triangle = Triangle::from([point_global; 3]);
                let barycentric_coords = [1. / 3.; 3].map(Into::into);

                (triangle, barycentric_coords)
            }
        }
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

    use crate::{
        algorithms::approx::Tolerance,
        geometry::{GlobalPath, SurfaceGeom},
    };

    #[test]
    fn point_from_surface_coords() {
        let surface = SurfaceGeom::Basic {
            u: GlobalPath::Line(Line::from_origin_and_direction(
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
