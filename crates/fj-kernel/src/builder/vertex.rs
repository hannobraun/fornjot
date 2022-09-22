use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface, SurfaceVertex};

/// API for building a [`SurfaceVertex`]
///
/// Also see [`SurfaceVertex::partial`].
pub struct PartialSurfaceVertex {
    /// The position of the [`SurfaceVertex`] on the [`Surface`]
    pub position: Point<2>,

    /// The surface that the [`SurfaceVertex`] is defined in
    pub surface: Option<Surface>,

    /// The global form of the [`SurfaceVertex`]
    ///
    /// Can be provided to the builder, if already available, or computed from
    /// the position on the [`Surface`].
    pub global_form: Option<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Provide a surface for the partial surface vertex
    pub fn with_surface(mut self, surface: Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Build the [`SurfaceVertex`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Finish building the [`SurfaceVertex`]
    pub fn build(self) -> SurfaceVertex {
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        let global_form = self.global_form.unwrap_or_else(|| {
            GlobalVertex::builder()
                .build_from_surface_and_position(&surface, self.position)
        });

        SurfaceVertex::new(self.position, surface, global_form)
    }
}

/// API for building a [`GlobalVertex`]
///
/// Also see [`GlobalVertex::builder`].
pub struct GlobalVertexBuilder;

impl GlobalVertexBuilder {
    /// Build a [`GlobalVertex`] from a curve and a position on that curve
    pub fn build_from_curve_and_position(
        &self,
        curve: &Curve,
        position: impl Into<Point<1>>,
    ) -> GlobalVertex {
        let position_surface = curve.path().point_from_path_coords(position);
        self.build_from_surface_and_position(curve.surface(), position_surface)
    }

    /// Build a [`GlobalVertex`] from a surface and a position on that surface
    pub fn build_from_surface_and_position(
        &self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> GlobalVertex {
        let position = surface.point_from_surface_coords(position);
        GlobalVertex::from_position(position)
    }
}
