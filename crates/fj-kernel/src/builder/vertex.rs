use fj_math::Point;

use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{Curve, GlobalVertex},
    partial::{
        HasPartial, PartialGlobalVertex, PartialSurfaceVertex, PartialVertex,
    },
    partial2::Partial,
};

/// Builder API for [`PartialVertex`]
pub trait VertexBuilder {
    /// Remove the surface form of the partial vertex, inferring it on build
    fn infer_surface_form(&mut self) -> &mut Self;
}

impl VertexBuilder for PartialVertex {
    fn infer_surface_form(&mut self) -> &mut Self {
        self.surface_form = PartialSurfaceVertex::default().into();
        self
    }
}

/// Builder API for [`PartialSurfaceVertex`]
pub trait SurfaceVertexBuilder {
    /// Infer the global form of the partial vertex
    fn infer_global_form(&mut self) -> &mut Self;
}

impl SurfaceVertexBuilder for PartialSurfaceVertex {
    fn infer_global_form(&mut self) -> &mut Self {
        self.global_form = GlobalVertex::partial().into();
        self
    }
}

/// Builder API for [`PartialGlobalVertex`]
pub trait GlobalVertexBuilder {
    /// Update partial global vertex from the given curve and position on it
    fn from_curve_and_position(
        curve: Partial<Curve>,
        position: impl Into<Point<1>>,
    ) -> Self;

    /// Update partial global vertex from the given surface and position on it
    fn from_surface_and_position(
        surface: &SurfaceGeometry,
        position: impl Into<Point<2>>,
    ) -> Self;
}

impl GlobalVertexBuilder for PartialGlobalVertex {
    fn from_curve_and_position(
        curve: Partial<Curve>,
        position: impl Into<Point<1>>,
    ) -> Self {
        let path = curve.read().path.expect(
            "Need path to create `GlobalVertex` from curve and position",
        );
        let surface = curve.read().surface.read().geometry.expect(
            "Need surface to create `GlobalVertex` from curve and position",
        );

        let position_surface = path.point_from_path_coords(position);

        Self::from_surface_and_position(&surface, position_surface)
    }

    fn from_surface_and_position(
        surface: &SurfaceGeometry,
        position: impl Into<Point<2>>,
    ) -> Self {
        Self {
            position: Some(surface.point_from_surface_coords(position)),
        }
    }
}
