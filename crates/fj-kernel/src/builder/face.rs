use fj_math::Point;

use crate::{
    objects::Cycle,
    partial::{HasPartial, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the [`PartialFace`] with an exterior polygon
    fn with_exterior_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;

    /// Update the [`PartialFace`] with an interior polygon
    fn with_interior_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;
}

impl FaceBuilder for PartialFace {
    fn with_exterior_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = self.surface().expect("Need surface to create polygon");

        self.with_exterior(
            Cycle::partial()
                .with_poly_chain_from_points(surface, points)
                .close_with_line_segment(),
        )
    }

    fn with_interior_polygon_from_points(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = self.surface().expect("Need surface to build polygon.");

        self.with_interiors([Cycle::partial()
            .with_poly_chain_from_points(surface, points)
            .close_with_line_segment()])
    }
}
