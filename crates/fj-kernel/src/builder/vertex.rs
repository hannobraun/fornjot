use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface, SurfaceVertex, Vertex};

/// API for building a [`Vertex`]
///
/// Also see [`Vertex::builder`].
#[derive(Default)]
pub struct PartialVertex {
    /// The position of the [`Vertex`] on the [`Curve`]
    pub position: Option<Point<1>>,

    /// The curve that the [`Vertex`] is defined in
    pub curve: Option<Curve>,

    /// The surface form of the [`Vertex`]
    ///
    /// Can be provided to the builder, if already available, or computed from
    /// the position on the [`Curve`].
    pub surface_form: Option<SurfaceVertex>,

    /// The global form of the [`Vertex`]
    ///
    /// Can be provided to the builder, if already available, or acquired
    /// through the surface form.
    pub global_form: Option<GlobalVertex>,
}

impl PartialVertex {
    /// Build the [`Vertex`] with the provided position
    pub fn with_position(mut self, position: impl Into<Point<1>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Build the [`Vertex`] with the provided curve
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = Some(curve);
        self
    }

    /// Build the [`Vertex`] with the provided surface form
    pub fn with_surface_form(mut self, surface_form: SurfaceVertex) -> Self {
        self.surface_form = Some(surface_form);
        self
    }

    /// Build the [`Vertex`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Finish building the [`Vertex`]
    pub fn build(self) -> Vertex {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.expect("Can't build `Vertex` without `Curve`");

        let surface_form = self.surface_form.unwrap_or_else(|| {
            SurfaceVertexBuilder {
                position: curve.path().point_from_path_coords(position),
                surface: *curve.surface(),
                global_form: self.global_form,
            }
            .build()
        });

        let global_form = *surface_form.global_form();

        Vertex::new(position, curve, surface_form, global_form)
    }
}

/// API for building a [`SurfaceVertex`]
///
/// Also see [`SurfaceVertex::builder`].
pub struct SurfaceVertexBuilder {
    /// The position of the [`SurfaceVertex`] on the [`Surface`]
    pub position: Point<2>,

    /// The surface that the [`SurfaceVertex`] is defined in
    pub surface: Surface,

    /// The global form of the [`SurfaceVertex`]
    ///
    /// Can be provided to the builder, if already available, or computed from
    /// the position on the [`Surface`].
    pub global_form: Option<GlobalVertex>,
}

impl SurfaceVertexBuilder {
    /// Build the [`SurfaceVertex`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Finish building the [`SurfaceVertex`]
    pub fn build(self) -> SurfaceVertex {
        let global_form = self.global_form.unwrap_or_else(|| {
            GlobalVertex::builder()
                .build_from_surface_and_position(&self.surface, self.position)
        });

        SurfaceVertex::new(self.position, self.surface, global_form)
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
