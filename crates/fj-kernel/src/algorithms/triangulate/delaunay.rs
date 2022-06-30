use fj_math::{Scalar, Triangle, Winding};
use spade::HasPosition;

use crate::geometry;

/// Create a Delaunay triangulation of all points
pub fn triangulate(
    points: Vec<geometry::LocalPoint<2>>,
) -> Vec<[geometry::LocalPoint<2>; 3]> {
    use spade::Triangulation as _;

    let triangulation = spade::DelaunayTriangulation::<_>::bulk_load(points)
        .expect("Inserted invalid values into triangulation");

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| *vertex.data());
        let orientation =
            Triangle::<2>::from_points([v0.local(), v1.local(), v2.local()])
                .winding_direction();

        let triangle = match orientation {
            Winding::Ccw => [v0, v1, v2],
            Winding::Cw => [v0, v2, v1],
        };

        triangles.push(triangle);
    }

    triangles
}

// Enables the use of `geometry::Point` in the triangulation.
impl HasPosition for geometry::LocalPoint<2> {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.local().u,
            y: self.local().v,
        }
    }
}
