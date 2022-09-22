use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface};

/// API for building a [`GlobalVertex`]
///
/// Also see [`GlobalVertex::builder`].
#[derive(Default)]
pub struct GlobalVertexBuilder {
    /// The position of the [`GlobalVertex`]
    pub position: Option<Point<3>>,
}

impl GlobalVertexBuilder {
    /// Provide a position
    pub fn with_position(mut self, position: impl Into<Point<3>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Build a [`GlobalVertex`] from a curve and a position on that curve
    pub fn build_from_curve_and_position(
        self,
        curve: &Curve,
        position: impl Into<Point<1>>,
    ) -> GlobalVertex {
        let position_surface = curve.path().point_from_path_coords(position);
        self.build_from_surface_and_position(curve.surface(), position_surface)
    }

    /// Build a [`GlobalVertex`] from a surface and a position on that surface
    pub fn build_from_surface_and_position(
        self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> GlobalVertex {
        let position = surface.point_from_surface_coords(position);
        GlobalVertex::from_position(position)
    }

    /// Build a full [`GlobalVertex`]
    pub fn build(self) -> GlobalVertex {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        GlobalVertex::from_position(position)
    }
}
