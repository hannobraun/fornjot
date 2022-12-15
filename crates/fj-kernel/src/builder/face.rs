use fj_math::Point;

use crate::{
    objects::Surface,
    partial::{Partial, PartialCycle, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the [`PartialFace`] with an exterior polygon
    fn update_exterior_as_polygon(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );

    /// Update the [`PartialFace`] with an interior polygon
    fn add_interior_polygon(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl FaceBuilder for PartialFace {
    fn update_exterior_as_polygon(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let cycle = PartialCycle::from_poly_chain(surface, points);
        self.exterior = Partial::from_partial(cycle);
    }

    fn add_interior_polygon(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let cycle = PartialCycle::from_poly_chain(surface, points);
        self.interiors = vec![Partial::from_partial(cycle)];
    }
}
