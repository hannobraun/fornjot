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
        let &surface = self.curve.surface();

        let global_form =
            GlobalVertex::builder().from_curve_and_position(&self.curve, point);

        let surface_form = SurfaceVertex::new(
            self.curve.path().point_from_path_coords(point),
            surface,
            global_form,
        );

        Vertex::new([0.], self.curve.clone(), surface_form, global_form)
    }
}

/// API for building a [`GlobalVertex`]
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
