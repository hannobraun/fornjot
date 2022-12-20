use fj_math::Point;

use crate::{
    objects::HalfEdge,
    partial::{Partial, PartialCycle, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the [`PartialFace`] with an exterior polygon
    fn update_exterior_as_polygon(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>>;

    /// Update the [`PartialFace`] with an interior polygon
    fn add_interior_polygon(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl FaceBuilder for PartialFace {
    fn update_exterior_as_polygon(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>> {
        self.exterior.write().update_as_polygon_from_points(points)
    }

    fn add_interior_polygon(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) {
        let mut cycle = PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        };
        cycle.update_as_polygon_from_points(points);

        self.interiors.push(Partial::from_partial(cycle));
    }
}
