use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{Partial, PartialCycle, PartialFace},
    storage::Handle,
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the [`PartialFace`] with an exterior polygon
    fn with_exterior_polygon_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;

    /// Update the [`PartialFace`] with an interior polygon
    fn with_interior_polygon_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;
}

impl FaceBuilder for PartialFace {
    fn with_exterior_polygon_from_points(
        mut self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let mut cycle = PartialCycle::default();
        cycle.with_poly_chain_from_points(surface, points);
        cycle.close_with_line_segment();

        self.exterior = Partial::from_partial(cycle);
        self
    }

    fn with_interior_polygon_from_points(
        mut self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let mut cycle = PartialCycle::default();
        cycle.with_poly_chain_from_points(surface, points);
        cycle.close_with_line_segment();

        self.interiors = vec![Partial::from_partial(cycle)];
        self
    }
}
