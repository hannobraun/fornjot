use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{
        Partial, PartialGlobalVertex, PartialSurfaceVertex, PartialVertex,
    },
};

/// Builder API for [`PartialVertex`]
pub trait VertexBuilder {
    /// Completely replace the surface in this vertex' object graph
    ///
    /// Please note that this operation will write to every partial object that
    /// the vertex references. If any of them were created from full objects,
    /// this will break the connection to those, meaning that building the
    /// partial objects won't result in those full objects again. This will be
    /// the case, even if those full objects already referenced the provided
    /// surface.
    fn replace_surface(&mut self, surface: impl Into<Partial<Surface>>);
}

impl VertexBuilder for PartialVertex {
    fn replace_surface(&mut self, surface: impl Into<Partial<Surface>>) {
        let surface = surface.into();
        self.surface_form.write().surface = surface;
    }
}

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
