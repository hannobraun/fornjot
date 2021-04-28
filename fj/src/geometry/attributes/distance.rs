use crate::geometry::shapes::Point;

/// Provides a signed distance function
pub trait Distance {
    fn distance(&self, point: Point<3>);
}
