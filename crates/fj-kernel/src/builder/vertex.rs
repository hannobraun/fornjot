use crate::partial::{PartialGlobalVertex, PartialSurfaceVertex};

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {}

impl SurfaceVertexBuilder for PartialSurfaceVertex {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {}

impl GlobalVertexBuilder for PartialGlobalVertex {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}
