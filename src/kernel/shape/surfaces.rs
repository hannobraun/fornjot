use crate::kernel::geometry::Surface;

use super::handle::{Handle, Storage};

/// API to access the surfaces of a shape
pub struct Surfaces;

impl Surfaces {
    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        Storage::new(surface).handle()
    }
}
