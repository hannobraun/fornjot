use geo::Polygon;

use super::point::TriangulationPoint;

pub struct ProjectedFace {
    pub polygon_from_half_edges: Polygon,
    pub points: Vec<TriangulationPoint>,
}
