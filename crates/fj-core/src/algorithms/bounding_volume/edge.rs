use fj_math::Aabb;

use crate::{geometry::curve::Curve, objects::HalfEdge};

impl super::BoundingVolume<2> for HalfEdge {
    fn aabb(&self) -> Option<Aabb<2>> {
        match self.curve() {
            Curve::Circle(_) => {
                // I don't currently have an example model to test this with.
                // This should change soon, and then this will panic  and can be
                // addressed.
                todo!("Computing AABB of arc is not supported yet")
            }
            Curve::Line(_) => {
                let points = self.boundary().map(|point_curve| {
                    self.curve().point_from_path_coords(point_curve)
                });

                Some(Aabb::<2>::from_points(points))
            }
        }
    }
}
