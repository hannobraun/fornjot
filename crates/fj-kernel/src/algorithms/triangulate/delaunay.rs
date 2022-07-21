use fj_math::{Point, Scalar, Triangle, Winding};
use spade::HasPosition;

use crate::local::Local;

/// Create a Delaunay triangulation of all points
pub fn triangulate(points: Vec<Local<Point<2>>>) -> Vec<[Local<Point<2>>; 3]> {
    use spade::Triangulation as _;

    let triangulation = spade::DelaunayTriangulation::<_>::bulk_load(points)
        .expect("Inserted invalid values into triangulation");

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| *vertex.data());
        let orientation = Triangle::<2>::from_points([
            *v0.local_form(),
            *v1.local_form(),
            *v2.local_form(),
        ])
        .winding_direction();

        let triangle = match orientation {
            Winding::Ccw => [v0, v1, v2],
            Winding::Cw => [v0, v2, v1],
        };

        triangles.push(triangle);
    }

    triangles
}

// Enables the use of `LocalPoint` in the triangulation.
impl HasPosition for Local<Point<2>> {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.local_form().u,
            y: self.local_form().v,
        }
    }
}
