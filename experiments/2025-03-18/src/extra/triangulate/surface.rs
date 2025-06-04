use fj_interop::Tolerance;
use fj_math::{Aabb, Point};

use crate::topology::surface::Surface;

pub fn triangulate_surface(
    surface: &Surface,
    boundary: &Aabb<2>,
    _: impl Into<Tolerance>,
) -> Vec<Point<2>> {
    surface.geometry.approximate(boundary)
}
