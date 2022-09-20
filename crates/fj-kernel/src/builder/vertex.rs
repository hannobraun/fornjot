use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface, SurfaceVertex, Vertex};

/// API for building a [`Vertex`]
///
/// Also see [`Vertex::builder`].
pub struct VertexBuilder {
    /// The curve that the [`Vertex`] is defined in
    pub curve: Curve,
}

impl VertexBuilder {
    /// Build a vertex from a curve position
    pub fn from_point(&self, point: impl Into<Point<1>>) -> Vertex {
        let point = point.into();

        let surface_form = SurfaceVertexBuilder {
            position: self.curve.path().point_from_path_coords(point),
            surface: *self.curve.surface(),
            global_form: None,
        }
        .build();

        let global_form = *surface_form.global_form();

        Vertex::new(point, self.curve.clone(), surface_form, global_form)
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
                .from_surface_and_position(&self.surface, self.position)
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
    pub fn from_curve_and_position(
        &self,
        curve: &Curve,
        position: impl Into<Point<1>>,
    ) -> GlobalVertex {
        let position_surface = curve.path().point_from_path_coords(position);
        self.from_surface_and_position(curve.surface(), position_surface)
    }

    /// Build a [`GlobalVertex`] from a surface and a position on that surface
    pub fn from_surface_and_position(
        &self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> GlobalVertex {
        let position = surface.point_from_surface_coords(position);
        GlobalVertex::from_position(position)
    }
}
