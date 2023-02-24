//! Intersection between a ray and an edge in 2D

use fj_math::Segment;

use crate::{
    algorithms::intersect::{HorizontalRayToTheRight, Intersect},
    geometry::path::Curve,
    objects::HalfEdge,
    storage::Handle,
};

use super::ray_segment::RaySegmentIntersection;

impl Intersect for (&HorizontalRayToTheRight<2>, &Handle<HalfEdge>) {
    type Intersection = RaySegmentIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, edge) = self;

        let line = match edge.curve() {
            Curve::Line(line) => line,
            Curve::Circle(_) => {
                todo!("Casting rays against circles is not supported yet")
            }
        };

        let points = edge
            .boundary()
            .map(|point| line.point_from_line_coords(point));
        let segment = Segment::from_points(points);

        (ray, &segment).intersect()
    }
}
