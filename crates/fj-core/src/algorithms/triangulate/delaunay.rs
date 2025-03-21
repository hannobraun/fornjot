use std::collections::BTreeMap;

use fj_math::{Point, Scalar, Triangle, Winding};
use spade::HasPosition;

use crate::{algorithms::approx::cycle::CycleApprox, topology::Handedness};

/// Create a Delaunay triangulation of all points
pub fn triangulate(
    cycles: impl IntoIterator<Item = CycleApprox>,
    coord_handedness: Handedness,
) -> Vec<[TriangulationPoint; 3]> {
    use spade::Triangulation as _;

    let mut triangulation = spade::ConstrainedDelaunayTriangulation::<_>::new();

    let mut points = BTreeMap::new();

    for cycle_approx in cycles {
        let mut handle_prev = None;

        for point in cycle_approx.points() {
            let handle = match points.get(&point) {
                Some(handle) => *handle,
                None => {
                    let handle = triangulation
                        .insert(TriangulationPoint {
                            point_surface: point.local_form,
                            point_global: point.global_form,
                        })
                        .expect("Inserted invalid point into triangulation");

                    points.insert(point, handle);

                    handle
                }
            };

            if let Some(handle_prev) = handle_prev {
                triangulation.add_constraint(handle_prev, handle);
            }

            handle_prev = Some(handle);
        }
    }

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| *vertex.data());
        let triangle = Triangle::<2>::from_points([
            v0.point_surface,
            v1.point_surface,
            v2.point_surface,
        ]);
        assert!(
            triangle.is_valid(),
            "Expecting triangles created by triangulation to be valid.",
        );

        let required_winding = match coord_handedness {
            Handedness::LeftHanded => Winding::Cw,
            Handedness::RightHanded => Winding::Ccw,
        };
        let Some(actual_winding) = triangle.winding() else {
            unreachable!(
                "Just asserted that the triangle is valid. Therefore it must \
                have a winding."
            );
        };

        let triangle = if actual_winding == required_winding {
            [v0, v1, v2]
        } else {
            [v0, v2, v1]
        };

        triangles.push(triangle);
    }

    triangles
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TriangulationPoint {
    pub point_surface: Point<2>,
    pub point_global: Point<3>,
}

// Enables the use of `LocalPoint` in the triangulation.
impl HasPosition for TriangulationPoint {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.point_surface.u,
            y: self.point_surface.v,
        }
    }
}
