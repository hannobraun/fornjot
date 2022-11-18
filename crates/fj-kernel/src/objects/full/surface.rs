use crate::geometry::surface::SurfaceGeometry;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Surface {
    geometry: SurfaceGeometry,
}

impl Surface {
    /// Construct a `Surface` from two paths that define its coordinate system
    pub fn new(geometry: SurfaceGeometry) -> Self {
        Self { geometry }
    }

    /// Access the surface's geometry
    pub fn geometry(&self) -> SurfaceGeometry {
        self.geometry
    }
}
