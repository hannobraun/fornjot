use nalgebra::Point;

/// Provides a signed distance function
pub trait Distance<const D: usize> {
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32;
}
