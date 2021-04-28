use nalgebra::Point;

/// Provides a signed distance function
pub trait Distance {
    fn distance(&self, point: Point<f32, 3>) -> f32;
}
