use nalgebra::point;
use parry2d_f64::{
    shape::Triangle,
    utils::point_in_triangle::{corner_direction, Orientation},
};
use spade::HasPosition;

use super::geometry::points::SurfacePoint;

/// Create a Delaunay triangulation of all points
pub fn triangulate(points: Vec<SurfacePoint>) -> Vec<Triangle> {
    use spade::Triangulation as _;

    let triangulation = spade::DelaunayTriangulation::<_>::bulk_load(points)
        .expect("Inserted invalid values into triangulation");

    let mut triangles = Vec::new();
    for triangle in triangulation.inner_faces() {
        let [v0, v1, v2] = triangle.vertices().map(|vertex| {
            let pos = vertex.position();
            point![pos.x, pos.y]
        });

        let triangle = match corner_direction(&v0, &v1, &v2) {
            Orientation::Ccw => [v0, v1, v2].into(),
            Orientation::Cw => [v0, v2, v1].into(),
            Orientation::None => {
                panic!(
                    "Triangle returned from triangulation isn't actually a\
                    triangle"
                );
            }
        };

        triangles.push(triangle);
    }

    triangles
}

// Enables the use of `SurfacePoint` in the triangulation.
impl HasPosition for SurfacePoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2 {
            x: self.value.x,
            y: self.value.y,
        }
    }
}
