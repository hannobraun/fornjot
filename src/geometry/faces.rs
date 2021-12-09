use parry3d_f64::shape::Triangle;

use crate::math::Point;

/// The faces of a shape
///
/// Right now, this is just the triangulated form of all faces. The plan is to
/// refactor this over time, to make it more similar to `Edges`.
pub struct Faces(pub Vec<Triangle>);

pub fn triangulate(vertices: &[Point]) -> Vec<Triangle> {
    let points: Vec<_> = vertices
        .iter()
        .map(|vertex| delaunator::Point {
            x: vertex.x,
            y: vertex.y,
        })
        .collect();

    let triangulation = delaunator::triangulate(&points);

    let mut triangles = Vec::new();
    for triangle in triangulation.triangles.chunks(3) {
        let i0 = triangle[0];
        let i1 = triangle[1];
        let i2 = triangle[2];

        triangles.push([vertices[i0], vertices[i2], vertices[i1]].into());
    }

    triangles
}
