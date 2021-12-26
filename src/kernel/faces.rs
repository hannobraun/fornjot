use parry3d_f64::{math::Isometry, shape::Triangle};

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
pub enum Face {
    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle>),
}

impl Face {
    pub fn triangles(&self, out: &mut Vec<Triangle>) {
        let Self::Triangles(triangles) = self;
        out.extend(triangles);
    }

    pub fn transform(&mut self, transform: &Isometry<f64>) {
        let Self::Triangles(triangles) = self;

        for triangle in triangles {
            *triangle = triangle.transformed(transform);
        }
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
