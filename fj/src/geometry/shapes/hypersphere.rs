use nalgebra::Point;

use crate::geometry::attributes::{Distance, SignedDistanceField};

/// An n-dimensional hypersphere
///
/// `Hypersphere` is typically used through one of its type aliases, like
/// [`Circle`] or [`Sphere`].
///
/// [`Circle`]: crate::geometry::shapes::Circle
/// [`Sphere`]: crate::geometry::shapes::Sphere
#[derive(Default)]
pub struct Hypersphere<const D: usize> {
    /// The radius of the hypersphere
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
    ///
    /// Returns a copy of `self`, with the radius replaced with `radius`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl<const D: usize> SignedDistanceField<D> for Hypersphere<D> {
    fn distance(&self, point: impl Into<Point<f32, D>>) -> Distance<D> {
        let point = point.into();

        Distance {
            point,
            distance: point.coords.magnitude() - self.radius,
        }
    }
}
