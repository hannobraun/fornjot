use fj_math::{Aabb, Vector};

use crate::{geometry::curve::Curve, objects::HalfEdge};

impl super::BoundingVolume<2> for HalfEdge {
    fn aabb(&self) -> Option<Aabb<2>> {
        match self.curve() {
            Curve::Circle(circle) => {
                // Just calculate the AABB of the whole circle. This is not the
                // most precise, but it should do for now.

                let center_to_min_max =
                    Vector::from([circle.radius(), circle.radius()]);

                Some(Aabb {
                    min: circle.center() - center_to_min_max,
                    max: circle.center() + center_to_min_max,
                })
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
