use crate::geometry::CurveAnchored;

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Curve {
    pub geometry: CurveAnchored,
}

impl Curve {
    pub fn line_from_vertices(vertices: [&Vertex; 2]) -> Self {
        let points = vertices.map(|vertex| vertex.point);
        let geometry = CurveAnchored::line_from_points(points);
        Self { geometry }
    }
}
