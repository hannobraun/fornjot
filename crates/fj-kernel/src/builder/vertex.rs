use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, SurfaceVertex, Vertex};

/// API for building a [`Vertex`]
pub struct VertexBuilder {
    curve: Curve,
}

impl VertexBuilder {
    /// Construct a new instance of `VertexBuilder`
    ///
    /// Also see [`Vertex::build`].
    pub fn new(curve: Curve) -> Self {
        Self { curve }
    }

    /// Build a vertex from a curve position
    pub fn from_point(&self, point: impl Into<Point<1>>) -> Vertex {
        let point = point.into();
        let &surface = self.curve.surface();

        let global_form =
            GlobalVertex::build().from_curve_and_position(&self.curve, point);

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
        let position_global =
            curve.global_form().path().point_from_path_coords(position);
        GlobalVertex::from_position(position_global)
    }
}
