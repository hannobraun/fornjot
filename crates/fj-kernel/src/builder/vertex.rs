use crate::partial::{PartialGlobalVertex, PartialSurfaceVertex};

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl SurfaceVertexBuilder for PartialSurfaceVertex {}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl GlobalVertexBuilder for PartialGlobalVertex {}
