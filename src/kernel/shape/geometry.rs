use crate::{
    kernel::geometry::{Curve, Surface},
    math::Point,
};

use super::{handle::Storage, ValidationResult};

/// API to access a shape's geometry
pub struct Geometry;

impl Geometry {
    /// Add a point to the shape
    pub fn add_point(&mut self, point: Point<3>) -> ValidationResult<Point<3>> {
        Ok(Storage::new(point).handle())
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> ValidationResult<Curve> {
        Ok(Storage::new(curve).handle())
    }

    /// Add a surface to the shape
    pub fn add_surface(
        &mut self,
        surface: Surface,
    ) -> ValidationResult<Surface> {
        Ok(Storage::new(surface).handle())
    }
}
