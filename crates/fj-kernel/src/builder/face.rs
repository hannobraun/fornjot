use fj_math::Point;

use crate::{
    objects::Cycle,
    partial::{Partial, PartialCycle, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior polygon, from the provided points
    fn add_interior_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Partial<Cycle> {
        let mut cycle = PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        };
        cycle.update_as_polygon_from_points(points);

        let cycle = Partial::from_partial(cycle);
        self.interiors.push(cycle.clone());

        cycle
    }
}
