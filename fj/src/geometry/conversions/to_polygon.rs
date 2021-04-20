use crate::geometry::shapes::Polygon;

pub trait ToPolygon {
    fn to_polygon(self, tolerance: f32) -> Polygon;
}

impl ToPolygon for Polygon {
    fn to_polygon(self, _tolerance: f32) -> Polygon {
        self
    }
}
