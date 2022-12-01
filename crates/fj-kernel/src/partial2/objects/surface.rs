use crate::geometry::surface::SurfaceGeometry;

/// A partial [`Surface`]
///
/// [`Surface`]: crate::objects::Surface
pub struct PartialSurface {
    /// The surface's geometry
    pub geometry: Option<SurfaceGeometry>,
}
