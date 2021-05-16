use nalgebra::Point;

use crate::geometry::attributes::Distance;

pub struct Hypersphere<const D: usize> {
    pub radius: f32,
}

impl<const D: usize> Hypersphere<D> {
    pub fn new() -> Self {
        Self { radius: 1.0 }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<const D: usize> Distance<D> for Hypersphere<D> {
    fn distance(&self, point: impl Into<Point<f32, D>>) -> f32 {
        let point = point.into();

        point.coords.magnitude() - self.radius
    }
}
