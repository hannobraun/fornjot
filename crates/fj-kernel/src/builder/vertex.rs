use fj_math::Point;

use crate::partial::{
    PartialGlobalVertex, PartialSurfaceVertex, PartialVertex,
};

/// Builder API for [`PartialVertex`]
pub trait VertexBuilder {}

impl VertexBuilder for PartialVertex {}

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {
    /// Infer the position of the surface vertex' global form
    ///
    /// Updates the global vertex referenced by this surface vertex with the
    /// inferred position, and also returns the position.
    fn infer_global_position(&mut self) -> Point<3>;
}

impl SurfaceVertexBuilder for PartialSurfaceVertex {
    fn infer_global_position(&mut self) -> Point<3> {
        let position_surface = self
            .position
            .expect("Can't infer global position without surface position");
        let surface = self
            .surface
            .read()
            .geometry
            .expect("Can't infer global position without surface geometry");

        let position_global =
            surface.point_from_surface_coords(position_surface);
        self.global_form.write().position = Some(position_global);

        position_global
    }
}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {}

impl GlobalVertexBuilder for PartialGlobalVertex {
    // No methods are currently defined. This trait serves as a placeholder, to
    // make it clear where to add such methods, once necessary.
}
