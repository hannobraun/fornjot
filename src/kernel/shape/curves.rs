use crate::kernel::geometry::Curve;

use super::handle::{Handle, Storage};

/// API to access the curves of a shape
pub struct Curves;

impl Curves {
    /// Add a curve to the shape
    pub fn add(&mut self, curve: Curve) -> Handle<Curve> {
        Storage::new(curve).handle()
    }
}
