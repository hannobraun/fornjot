use crate::{
    geometry::surface::SurfaceGeometry, objects::Surface,
    partial2::PartialObject,
};

/// A partial [`Surface`]
#[derive(Clone)]
pub struct PartialSurface {
    /// The surface's geometry
    pub geometry: Option<SurfaceGeometry>,
}

impl PartialObject for PartialSurface {
    type Full = Surface;
}
