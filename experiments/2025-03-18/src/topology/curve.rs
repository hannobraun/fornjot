use crate::geometry::CurveGeometry;

use super::vertex::Vertex;

pub struct Curve {
    pub geometry: Box<dyn CurveGeometry>,
}

impl Curve {
    pub fn line_from_vertices(_: [&Vertex; 2]) -> Self {
        Self {
            geometry: Box::new(()),
        }
    }
}
