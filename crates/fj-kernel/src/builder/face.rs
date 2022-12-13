use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{Partial, PartialCycle, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the [`PartialFace`] with an exterior polygon
    fn with_exterior_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );

    /// Update the [`PartialFace`] with an interior polygon
    fn with_interior_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl FaceBuilder for PartialFace {
    fn with_exterior_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let mut cycle = PartialCycle::default();
        cycle.with_poly_chain_from_points(surface, points);
        cycle.close_with_line_segment();

        self.exterior = Partial::from_partial(cycle);
    }

    fn with_interior_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let mut cycle = PartialCycle::default();
        cycle.with_poly_chain_from_points(surface, points);
        cycle.close_with_line_segment();

        self.interiors = vec![Partial::from_partial(cycle)];
    }
}
