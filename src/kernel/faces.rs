use parry3d_f64::shape::Triangle;

use crate::math::Point;

/// The faces of a shape
///
/// Right now, this is just the triangulated form of all faces. The plan is to
/// refactor this over time, to make it more similar to `Edges`.
pub enum Faces {
    /// The faces
    Faces(Vec<Face>),

    /// The faces are only available in the form of triangles
    ///
    /// This variant exists for a transitionary phase, as `Faces` is refactored
    /// to be more structured.
    Triangles(Vec<Triangle>),
}

impl Faces {
    pub fn triangles(&self, out: &mut Vec<Triangle>) {
        match self {
            Self::Faces(faces) => {
                for face in faces {
                    out.extend(&face.0);
                }
            }
            Self::Triangles(triangles) => {
                out.extend(triangles);
            }
        }
    }
}

/// A face of a shape
///
/// Right now, faces are represented as a collection of triangles. This is a
/// temporary state. The plan is to eventually represent faces as a geometric
/// surface, bounded by edges.
pub struct Face(pub Vec<Triangle>);

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
