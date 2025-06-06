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
    let surface_points = surface.geometry.approximate(boundary);

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

    let mut all_points = surface_points.clone();
    all_points.extend(boundary_points);

    let triangles = triangles(
        [],
        all_points.into_iter().map(|point_surface| {
            let point_global = surface.geometry.point_from_local(point_surface);
            TriangulationPoint {
                point_surface,
                point_global,
            }
        }),
    );

    (surface_points, triangles)
}
