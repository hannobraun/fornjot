use fj_interop::Tolerance;
use fj_math::{Aabb, Point};

use crate::{
    extra::triangulate::{TriangulationPoint, delaunay::triangles},
    topology::surface::Surface,
};

pub fn triangulate_surface(
    surface: &Surface,
    boundary: &Aabb<2>,
    _: impl Into<Tolerance>,
) -> (Vec<Point<2>>, Vec<[TriangulationPoint; 3]>) {
    let mut points = surface.geometry.approximate(boundary);

    let boundary_points = {
        let [[min_u, min_v], [max_u, max_v]] =
            [boundary.min, boundary.max].map(|point| point.coords.components);

        [
            [min_u, min_v],
            [min_u, max_v],
            [max_u, min_v],
            [max_u, max_v],
        ]
        .map(Point::from)
    };

    points.extend(boundary_points);

    let triangles = triangles(
        [],
        points.iter().copied().map(|point_surface| {
            let point_global = surface.geometry.point_from_local(point_surface);
            TriangulationPoint {
                point_surface,
                point_global,
            }
        }),
    );

    (points, triangles)
}
