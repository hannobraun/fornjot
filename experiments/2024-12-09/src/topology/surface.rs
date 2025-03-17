use std::fmt;

use crate::geometry::SurfaceGeometry;

/// # A surface
///
/// Surfaces are infinite 2D objects in 3D space. They are what defines faces,
/// which are bounded sections on a surface.
///
/// Surfaces own a reference to an implementation of `SurfaceGeometry`, which is
/// what defines them. So far, only planes are supported though.
pub struct Surface {
    pub geometry: Box<dyn SurfaceGeometry>,
}

impl fmt::Debug for Surface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Surface")
            .field("geometry", &"Box<dyn SurfaceGeometry>")
            .finish()
    }
}
