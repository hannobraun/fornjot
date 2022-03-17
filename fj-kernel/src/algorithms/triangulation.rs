use fj_math::Scalar;
use parry2d_f64::utils::point_in_triangle::{corner_direction, Orientation};
use spade::HasPosition;

use crate::geometry;

/// Create a Delaunay triangulation of all points
pub fn triangulate(
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

// Enables the use of `SurfacePoint` in the triangulation.
impl HasPosition for geometry::Point<2> {
    type Scalar = Scalar;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.native().u,
            y: self.native().v,
        }
    }
}
