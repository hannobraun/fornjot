pub mod edges;
pub mod faces;
pub mod vertices;

use self::{edges::Edges, faces::Faces};

/// A placeholder struct that will be filled with life later
pub struct Shape {
    pub edges: Edges,
    pub faces: Faces,
}
