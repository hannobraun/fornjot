//! Intersection between a ray and an edge in 2D

use fj_math::Segment;

use crate::{
    algorithms::intersect::{HorizontalRayToTheRight, Intersect},
    objects::{CurveKind, Edge},
};

use super::ray_segment::RaySegmentIntersection;

impl Intersect for (&HorizontalRayToTheRight<2>, &Edge) {
    type Intersection = RaySegmentIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, edge) = self;

        let line = match edge.curve().kind() {
            CurveKind::Line(line) => line,
            CurveKind::Circle(_) => {
                todo!("Casting rays against circles is not supported yet")
            }
        };

        let points = edge.vertices().get().map(|vertex| {
            let point = vertex.position();
            line.point_from_line_coords(point)
        });
        let segment = Segment::from_points(points);

        (ray, &segment).intersect()
    }
}
