use parry3d_f64::{math::Isometry, shape::Triangle};

use crate::math::Point;

use super::edges::Edges;

/// The faces of a shape
pub struct Faces(pub Vec<Face>);

impl Faces {
    pub fn triangles(&self, tolerance: f64, out: &mut Vec<Triangle>) {
        for face in &self.0 {
            face.triangles(tolerance, out);
        }
    }
}

/// A face of a shape
pub enum Face {
    /// A face of a shape
    ///
    /// A face is a section of a surface, bounded by edges. At this point, the
    /// surface is implicit, and assumed to be the x-y plane.
    Face {
        /// The edges that bound the face
        edges: Edges,
    },

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<Triangle>),
}

impl Face {
    pub fn triangles(&self, tolerance: f64, out: &mut Vec<Triangle>) {
        match self {
            Self::Face { edges } => {
                // TASK: This only works for faces that are convex and have no
                //       holes. These limitations should be lifted, ideally. At
                //       least, the presence of either of these should cause a
                //       panic, instead of incorrect results.

                let vertices = edges.approx_vertices(tolerance);
                let all_triangles = &triangulate(&vertices);
                out.extend(all_triangles);
            }
            Self::Triangles(triangles) => out.extend(triangles),
        }
    }

    pub fn transform(&mut self, transform: &Isometry<f64>) {
        match self {
            Self::Face { edges: _ } => {
                // TASK: Implement.
                todo!()
            }
            Self::Triangles(triangles) => {
                for triangle in triangles {
                    *triangle = triangle.transformed(transform);
                }
            }
        }
    }
}

/// Create a Delaunay triangulation of all vertices
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
