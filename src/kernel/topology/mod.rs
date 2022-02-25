pub mod edges;
pub mod faces;
pub mod vertices;

use self::{edges::Edges, faces::Faces, vertices::Vertices};

/// A placeholder struct that will be filled with life later
pub struct Shape {
    pub vertices: Vertices,
    pub edges: Edges,
    pub faces: Faces,
}
