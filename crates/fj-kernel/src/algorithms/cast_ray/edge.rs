use fj_math::Segment;

use crate::objects::{CurveKind, Edge};

use super::{CastRay, HorizontalRayToTheRight};

impl CastRay<2> for Edge {
    type Hit = <Segment<2> as CastRay<2>>::Hit;

    fn cast_ray(&self, ray: HorizontalRayToTheRight<2>) -> Option<Self::Hit> {
        let line = match self.curve().kind() {
            CurveKind::Line(line) => line,
            CurveKind::Circle(_) => {
                todo!("Casting rays against circles is not supported yet")
            }
        };

        let points = self.vertices().expect_vertices().map(|vertex| {
            let point = vertex.position();
            line.point_from_line_coords(point)
        });
        let segment = Segment::from_points(points);

        segment.cast_ray(ray)
    }
}
