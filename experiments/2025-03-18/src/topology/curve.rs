use fj_math::Line;

use crate::geometry::AnchoredCurveGeometry;

use super::vertex::Vertex;

pub struct Curve {
    pub geometry: AnchoredCurveGeometry,
}

impl Curve {
    pub fn line_from_vertices(vertices: [&Vertex; 2]) -> Self {
        let points = vertices.map(|vertex| vertex.point);
        let (line, _) = Line::from_points(points);

        Self {
            geometry: AnchoredCurveGeometry {
                origin: line.origin(),
                floating: Box::new(line),
            },
        }
    }
}
