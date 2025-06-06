use fj_interop::Tolerance;
use fj_math::{Aabb, Point};

use crate::topology::surface::Surface;

pub fn triangulate_surface(
    surface: &Surface,
    boundary: &Aabb<2>,
    _: impl Into<Tolerance>,
) -> Vec<Point<2>> {
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

    points
}
