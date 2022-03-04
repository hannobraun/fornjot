use crate::math::Point;

use super::VerticesInner;

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
        self.vertices
            .add(&point.into(), point)
            .expect("Error adding vertex");
        Vertex(point)
    }
}

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Uniqueness
///
/// You **MUST NOT** construct a new instance of `Vertex` that represents an
/// already existing vertex. If there already exists a vertex and you need a
/// `Vertex` instance to refer to it, acquire one by copying or converting the
/// existing `Vertex` instance.
///
/// Every time you create a `Vertex` instance, you might do so using a point you
/// have computed. When doing this for an existing vertex, you run the risk of
/// computing a slightly different point, due to floating point accuracy issues.
/// The resulting `Vertex` will then no longer be equal to the existing `Vertex`
/// instance that refers to the same vertex, which will cause bugs.
///
/// This can be prevented outright by never creating a new `Vertex` instance
/// for an existing vertex. Hence why this is strictly forbidden.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex(Point<3>);

impl Vertex {
    /// Access the point that defines this vertex
    pub fn point(&self) -> Point<3> {
        self.0
    }
}
