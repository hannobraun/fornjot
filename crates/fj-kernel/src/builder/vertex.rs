use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface, SurfaceVertex};

/// A partial [`SurfaceVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Default)]
pub struct PartialSurfaceVertex {
    /// The position of the [`SurfaceVertex`] in the [`Surface`]
    ///
    /// Must be provided before [`PartialSurfaceVertex::build`] is called.
    pub position: Option<Point<2>>,

    /// The surface that the [`SurfaceVertex`] is defined in
    ///
    /// Must be provided before [`PartialSurfaceVertex::build`] is called.
    pub surface: Option<Surface>,

    /// The global form of the [`SurfaceVertex`]
    ///
    /// Can be provided, if already available, or computed from the position on
    /// the [`Surface`].
    pub global_form: Option<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Provide a position for the partial surface vertex
    pub fn with_position(mut self, position: impl Into<Point<2>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Provide a surface for the partial surface vertex
    pub fn with_surface(mut self, surface: Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Provide a global form for the partial surface vertex
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    ///
    /// # Panics
    ///
    /// Panics, if no position has been provided.
    ///
    /// Panics, if no surface has been provided.
    pub fn build(self) -> SurfaceVertex {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        let global_form = self.global_form.unwrap_or_else(|| {
            GlobalVertex::builder()
                .build_from_surface_and_position(&surface, position)
        });

        SurfaceVertex::new(position, surface, global_form)
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
