use crate::{
    approx::Tolerance,
    geometry::{
        Geometry, Path,
        traits::{GenPolyline, GenTriMesh},
    },
    math::{Aabb, Point, Scalar, Triangle, Vector},
};

/// # A surface that is a curve, swept along a path
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SweptCurve {
    /// The u-axis of the surface
    pub u: Path<3>,

    /// The v-axis of the surface
    pub v: Vector<3>,
}

impl GenTriMesh for SweptCurve {
    fn origin(&self, _: &Geometry) -> Point<3> {
        self.u.origin()
    }

    fn triangle_at(
        &self,
        point_surface: Point<2>,
        tolerance: Tolerance,
        _: &Geometry,
    ) -> (Triangle<3>, [Scalar; 3]) {
        let [a, b] = self
            .u
            .line_segment_at(Point::from([point_surface.u]), tolerance)
            .points
            .map(|point_global| point_global + self.v * point_surface.v);

        let c = a + (b - a) / 2.;
        let triangle = Triangle::from([a, b, c]);

        let barycentric_coords = [1. / 3.; 3].map(Into::into);
        (triangle, barycentric_coords)
    }

    fn generate_tri_mesh(
        &self,
        boundary: Aabb<2>,
        tolerance: Tolerance,
        _: &Geometry,
    ) -> Vec<Point<2>> {
        let boundary_curve = [[boundary.min.u], [boundary.max.u]];
        let points_curve =
            self.u.generate_polyline(boundary_curve.into(), tolerance);

        points_curve
            .iter()
            .copied()
            .map(|point| [point.t, Scalar::ZERO])
            .chain(
                points_curve
                    .iter()
                    .copied()
                    .map(|point| [point.t, self.v.magnitude()]),
            )
            .map(Point::from)
            .collect()
    }
}
