use fj_math::Point;

use crate::{
    objects::{Curve, Surface},
    partial::{MaybePartial, PartialGlobalVertex},
};

/// Builder API for [`PartialGlobalVertex`]
#[allow(clippy::wrong_self_convention)]
pub trait GlobalVertexBuilder {
    /// Update partial global vertex from the given curve and position on it
    fn update_from_curve_and_position(
        self,
        curve: impl Into<MaybePartial<Curve>>,
        position: impl Into<Point<1>>,
    ) -> Self;

    /// Update partial global vertex from the given surface and position on it
    fn from_surface_and_position(
        self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> Self;
}

impl GlobalVertexBuilder for PartialGlobalVertex {
    fn update_from_curve_and_position(
        self,
        curve: impl Into<MaybePartial<Curve>>,
        position: impl Into<Point<1>>,
    ) -> Self {
        let curve = curve.into().into_partial();

        let path = curve.path().expect(
            "Need path to create `GlobalVertex` from curve and position",
        );
        let surface = curve.surface().expect(
            "Need surface to create `GlobalVertex` from curve and position",
        );

        let position_surface = path.point_from_path_coords(position);
        self.from_surface_and_position(&surface, position_surface)
    }

    fn from_surface_and_position(
        self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> Self {
        self.with_position(Some(surface.point_from_surface_coords(position)))
    }
}
