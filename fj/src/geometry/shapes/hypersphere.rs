use nalgebra::Point;

use crate::geometry::{
    aabb::Aabb,
    attributes::{BoundingVolume, Surface, SurfaceSample},
};

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

impl<const D: usize> BoundingVolume<D> for Hypersphere<D> {
    fn aabb(&self) -> Aabb<D> {
        Aabb {
            min: [-self.radius; D].into(),
            max: [self.radius; D].into(),
        }
    }
}

impl<const D: usize> Surface<D> for Hypersphere<D> {
    fn surface(&self, point: impl Into<Point<f32, D>>) -> SurfaceSample {
        let point = point.into();

        SurfaceSample {
            distance: point.coords.magnitude() - self.radius,
        }
    }
}
