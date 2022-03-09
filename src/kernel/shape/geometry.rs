use crate::kernel::geometry::{Curve, Surface};

use super::handle::{Handle, Storage};

/// API to access a shape's geometry
pub struct Geometry;

impl Geometry {
    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        Storage::new(curve).handle()
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        Storage::new(surface).handle()
    }
}
