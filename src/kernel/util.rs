use nalgebra::point;
use parry2d_f64::{
    shape::Triangle,
    utils::point_in_triangle::{corner_direction, Orientation},
};

use crate::math::Point;

/// Create a Delaunay triangulation of all vertices
pub fn triangulate(vertices: &[Point<2>]) -> Vec<Triangle> {
    use spade::Triangulation as _;

    let points: Vec<_> = vertices
        .iter()
        .map(|vertex| spade::Point2 {
            x: vertex.x,
            y: vertex.y,
        })
        .collect();

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
