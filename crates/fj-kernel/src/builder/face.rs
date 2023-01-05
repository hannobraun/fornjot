use fj_math::Point;

use crate::{
    objects::HalfEdge,
    partial::{Partial, PartialCycle, PartialFace},
};

use super::CycleBuilder;

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Update the face exterior as a polygon from the provided points
    fn update_exterior_as_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>>;

    /// Update the face exterior as a triangle, from 3D points
    ///
    /// Uses the three points to infer a plane that is used as the surface.
    ///
    /// # Implementation Note
    ///
    /// This method is probably just temporary, and will be generalized into a
    /// "update as polygon from global points" method sooner or later. For now,
    /// I didn't want to deal with the question of how to infer the surface, and
    /// how to handle points that don't fit that surface.
    fn update_exterior_as_triangle_from_global_points(
        &mut self,
        points: [impl Into<Point<3>>; 3],
    ) -> [Partial<HalfEdge>; 3];

    /// Add an interior polygon, from the provided points
    fn add_interior_polygon(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    );
}

impl FaceBuilder for PartialFace {
    fn update_exterior_as_polygon_from_points(
        &mut self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>> {
        self.exterior.write().update_as_polygon_from_points(points)
    }

    fn update_exterior_as_triangle_from_global_points(
        &mut self,
        points_global: [impl Into<Point<3>>; 3],
    ) -> [Partial<HalfEdge>; 3] {
        self.exterior
            .write()
            .update_as_triangle_from_global_points(points_global)
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
