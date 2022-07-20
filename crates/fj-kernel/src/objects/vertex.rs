use std::hash::Hash;

use fj_math::Point;

use super::GlobalVertex;

/// A vertex
///
/// `Vertex` is defined in terms of a 1-dimensional position on a curve. If you
/// need the 3D position of a vertex, you can use [`Vertex::global`], to get
/// access of the global form of a vertex.
///
/// # Implementation Note
///
/// Since `Vertex` is defined in terms of the curve it lies on, a reference to
/// that curve should be available here. As of this writing, this reference
/// still lives in [`Edge`].
///
/// [`Edge`]: super::Edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {
    position: Point<1>,
    global: GlobalVertex,
}

impl Vertex {
    /// Construct an instance of `Vertex`
    pub fn new(position: impl Into<Point<1>>, global: GlobalVertex) -> Self {
        let position = position.into();
        Self { position, global }
    }

    /// The position of the vertex on the curve
    pub fn position(&self) -> Point<1> {
        self.position
    }

    /// The global form of this vertex
    pub fn global(&self) -> &GlobalVertex {
        &self.global
    }
}
