use crate::geometry::SurfaceGeometry;

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Surface {
    geometry: SurfaceGeometry,
}

impl Surface {
    /// Construct an instance of `Surface`
    pub fn new(geometry: SurfaceGeometry) -> Self {
        Self { geometry }
    }

    /// Access the surface's geometry
    pub fn geometry(&self) -> SurfaceGeometry {
        self.geometry
    }
}
