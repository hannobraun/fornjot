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

        let global_form = GlobalVertex::from_position(
            self.curve
                .global_form()
                .path()
                .point_from_path_coords(point),
        );
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
