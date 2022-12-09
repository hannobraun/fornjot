use crate::partial::{
    PartialGlobalVertex, PartialSurfaceVertex, PartialVertex,
};

/// Builder API for [`PartialVertex`]
pub trait VertexBuilder {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}

impl VertexBuilder for PartialVertex {}

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {
    /// Infer the position of the surface vertex' global form
    fn infer_global_position(&mut self) -> &mut Self;
}

impl SurfaceVertexBuilder for PartialSurfaceVertex {
    fn infer_global_position(&mut self) -> &mut Self {
        let position = self
            .position
            .expect("Can't infer global position without surface position");
        let surface = self
            .surface
            .read()
            .geometry
            .expect("Can't infer global position without surface geometry");

        self.global_form.write().position =
            Some(surface.point_from_surface_coords(position));

        self
    }
}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {}

impl GlobalVertexBuilder for PartialGlobalVertex {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}
