pub mod edges;
pub mod faces;
pub mod vertices;

use self::{edges::Edges, faces::Faces, vertices::Vertices};

/// The boundary representation of a shape
pub struct Shape {
    pub vertices: Vertices,
    pub edges: Edges,
    pub faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            vertices: Vertices(Vec::new()),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }
}
