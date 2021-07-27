use nalgebra::{Point, Unit};

use crate::geometry::{
    aabb::Aabb,
    traits::{BoundingVolume, Geometry, Sample},
};

/// An n-dimensional hypersphere
///
/// Hypersphere is typically used through one of its type aliases, like
/// [`Circle`] or [`Sphere`].
///
/// [`Circle`]: crate::geometry::shapes::Circle
/// [`Sphere`]: crate::geometry::shapes::Sphere
#[derive(Default)]
pub struct Hypersphere<const D: usize> {
    pub radius: f32,
}

impl<const D: usize> Hypersphere<D> {
    /// Create a new hypersphere
    ///
    /// The radius is initially set to `1.0`.
    pub fn new() -> Self {
        Self { radius: 1.0 }
    }

    /// Update radius
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

impl<const D: usize> Geometry<D> for Hypersphere<D> {
    fn sample(&self, point: impl Into<Point<f32, D>>) -> Sample<D> {
        let point = point.into();

        Sample {
            point,
            distance: point.coords.magnitude() - self.radius,
            normal: Unit::new_normalize(point.coords),
        }
    }
}
