use fj_math::Line;

use crate::geometry::CurveGeometry;

use super::vertex::Vertex;

pub struct Curve {
    pub geometry: Box<dyn CurveGeometry>,
}

impl Curve {
    pub fn line_from_vertices(vertices: [&Vertex; 2]) -> Self {
        let points = vertices.map(|vertex| vertex.point);
        let (line, _) = Line::from_points(points);

        Self {
            geometry: Box::new(line),
        }
    }
}
