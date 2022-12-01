use crate::{
    geometry::surface::SurfaceGeometry, objects::Surface,
    partial2::PartialObject,
};

/// A partial [`Surface`]
///
/// [`Surface`]: crate::objects::Surface
pub struct PartialSurface {
    /// The surface's geometry
    pub geometry: Option<SurfaceGeometry>,
}

impl PartialObject for PartialSurface {
    type Full = Surface;
}
