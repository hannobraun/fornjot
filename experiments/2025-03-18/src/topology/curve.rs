use crate::geometry::AnchoredCurve;

use super::vertex::Vertex;

#[derive(Debug)]
pub struct Curve {
    pub geometry: AnchoredCurve,
}

impl Curve {
    pub fn line_from_vertices(vertices: [&Vertex; 2]) -> Self {
        let points = vertices.map(|vertex| vertex.point);
        let geometry = AnchoredCurve::line_from_points(points);
        Self { geometry }
    }
}
