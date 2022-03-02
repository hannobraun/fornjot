pub mod edges;
pub mod faces;
pub mod vertices;

use crate::math::Point;

use self::{edges::Edges, faces::Faces, vertices::Vertices};

/// The boundary representation of a shape
pub struct Shape {
    vertices: Vec<Point<3>>,

    pub edges: Edges,
    pub faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }

    /// Access and modify the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            vertices: &mut self.vertices,
        }
    }
}
