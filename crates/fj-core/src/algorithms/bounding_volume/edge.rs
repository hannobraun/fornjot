use fj_math::Aabb;

use crate::objects::HalfEdge;

impl super::BoundingVolume<2> for HalfEdge {
    fn aabb(&self) -> Option<Aabb<2>> {
        let points = self.boundary().map(|point_curve| {
            self.curve().point_from_path_coords(point_curve)
        });

        Some(Aabb::<2>::from_points(points))
    }
}
