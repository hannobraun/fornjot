use parry3d_f64::shape::Triangle;

use crate::math::Point;

/// The faces of a shape
pub struct Faces(pub Vec<Face>);

impl Faces {
    pub fn triangles(&self, out: &mut Vec<Triangle>) {
        for face in &self.0 {
            face.triangles(out);
        }
    }
}

/// A face of a shape
///
/// Right now, faces are represented as a collection of triangles. This is a
/// temporary state. The plan is to eventually represent faces as a geometric
/// surface, bounded by edges.
pub struct Face(pub Vec<Triangle>);

impl Face {
    pub fn triangles(&self, out: &mut Vec<Triangle>) {
        out.extend(&self.0);
    }
}

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
