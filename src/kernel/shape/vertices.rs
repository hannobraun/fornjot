use crate::{kernel::topology::vertices::Vertex, math::Point};

use super::{handle::Handle, VerticesInner};

/// The vertices of a shape
pub struct Vertices<'r> {
    pub(super) vertices: &'r mut VerticesInner,
}

impl Vertices<'_> {
    /// Create a vertex
    ///
    /// The caller must make sure to uphold all rules regarding vertex
    /// uniqueness.
    ///
    /// # Implementation note
    ///
    /// This method is the only means to create `Vertex` instances, outside of
    /// unit tests. That puts this method is in a great position to enforce
    /// vertex uniqueness rules, instead of requiring the user to uphold those.
    pub fn create(&mut self, point: Point<3>) -> Vertex {
        let handle = Handle::new(point);

        self.vertices
            .add(&point.into(), handle.inner())
            .expect("Error adding vertex");

        Vertex(handle)
    }
}
