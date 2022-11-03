use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Surface},
    partial::{
        HasPartial, MaybePartial, PartialGlobalVertex, PartialSurfaceVertex,
    },
};

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {
    /// Infer the global form of the partial vertex
    fn infer_global_form(self) -> Self;
}

impl SurfaceVertexBuilder for PartialSurfaceVertex {
    fn infer_global_form(self) -> Self {
        self.with_global_form(Some(GlobalVertex::partial()))
    }
}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {
    /// Update partial global vertex from the given curve and position on it
    fn update_from_curve_and_position(
        self,
        curve: impl Into<MaybePartial<Curve>>,
        position: impl Into<Point<1>>,
    ) -> Self;

    /// Update partial global vertex from the given surface and position on it
    fn update_from_surface_and_position(
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
        self.update_from_surface_and_position(&surface, position_surface)
    }

    fn update_from_surface_and_position(
        self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> Self {
        self.with_position(Some(surface.point_from_surface_coords(position)))
    }
}
