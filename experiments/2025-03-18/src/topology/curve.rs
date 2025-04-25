use crate::geometry::AnchoredCurve;

use super::vertex::Vertex;

pub struct Curve {
    pub geometry: AnchoredCurve,
}

impl Curve {
    pub fn line_from_vertices(vertices: [&Vertex; 2]) -> Self {
        let points = vertices.map(|vertex| vertex.point);

        Self {
            geometry: AnchoredCurve::line_from_points(points),
        }
    }
}
