use fj_debug::DebugInfo;
use fj_math::{Scalar, Triangle};
use parry2d_f64::utils::point_in_triangle::{corner_direction, Orientation};
use spade::HasPosition;

use crate::{geometry, shape::Shape};

/// Triangulate a shape
pub fn triangulate(
    mut shape: Shape,
    tolerance: Scalar,
    out: &mut Vec<Triangle<3>>,
    debug_info: &mut DebugInfo,
) {
    for face in shape.topology().faces() {
        face.get().triangles(tolerance, out, debug_info);
    }
}

/// Create a Delaunay triangulation of all points
pub fn delaunay(
    points: Vec<geometry::Point<2>>,
) -> Vec<[geometry::Point<2>; 3]> {
    use spade::Triangulation as _;

    let triangulation = spade::DelaunayTriangulation::<_>::bulk_load(points)
        .expect("Inserted invalid values into triangulation");

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| *vertex.data());
        let orientation = corner_direction(
            &v0.native().to_na(),
            &v1.native().to_na(),
            &v2.native().to_na(),
        );

        let triangle = match orientation {
            Orientation::Ccw => [v0, v1, v2],
            Orientation::Cw => [v0, v2, v1],
            Orientation::None => {
                panic!(
                    "Triangle returned from triangulation isn't actually a \
                    triangle"
                );
            }
        };

        triangles.push(triangle);
    }

    triangles
}

// Enables the use of `geometry::Point` in the triangulation.
impl HasPosition for geometry::Point<2> {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.native().u,
            y: self.native().v,
        }
    }
}
