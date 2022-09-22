use fj_math::Point;

use crate::objects::{Curve, GlobalVertex, Surface};

/// A partial [`GlobalVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Default)]
pub struct PartialGlobalVertex {
    /// The position of the [`GlobalVertex`]
    ///
    /// Must be provided before [`PartialGlobalVertex::build`] is called.
    pub position: Option<Point<3>>,
}

impl PartialGlobalVertex {
    /// Provide a position for the partial global vertex
    pub fn with_position(mut self, position: impl Into<Point<3>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Update partial global vertex from the given curve and position on it
    pub fn from_curve_and_position(
        self,
        curve: &Curve,
        position: impl Into<Point<1>>,
    ) -> Self {
        let position_surface = curve.path().point_from_path_coords(position);
        self.from_surface_and_position(curve.surface(), position_surface)
    }

    /// Update partial global vertex from the given surface and position on it
    pub fn from_surface_and_position(
        mut self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> Self {
        self.position = Some(surface.point_from_surface_coords(position));
        self
    }

    /// Build a full [`GlobalVertex`] from the partial global vertex
    pub fn build(self) -> GlobalVertex {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        GlobalVertex::from_position(position)
    }
}
