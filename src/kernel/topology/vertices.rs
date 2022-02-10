use crate::math::Point;

/// The vertices of a shape
pub struct Vertices(pub Vec<Vertex<3>>);

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
#[derive(Clone, Copy, Debug)]
pub struct Vertex<const D: usize> {
    location: Point<D>,
}

impl Vertex<3> {
    /// Create a vertex at the given location
    ///
    /// This method **MUST NOT** be used to construct a new instance of `Vertex`
    /// that represents an already existing vertex. If there already exists a
    /// vertex and you need a `Vertex` instance to refer to it, acquire one by
    /// copying the existing `Vertex` instance.
    ///
    /// Every time you create a `Vertex` instance, you might do so using a point
    /// you have computed. When doing this for an existing vertex, you run the
    /// risk of computing a slightly different point, due to floating point
    /// accuracy issues. The resulting `Vertex` will then no longer be equal to
    /// the existing `Vertex` instance that refers to the same vertex, which
    /// will surely cause bugs.
    ///
    /// This can be prevented outright by never creating a new `Vertex` instance
    /// for an existing vertex. Hence why this is strictly forbidden.
    pub fn create_at(location: Point<3>) -> Self {
        Self { location }
    }
}

impl<const D: usize> Vertex<D> {
    /// Access the location of this vertex
    pub fn location(&self) -> &Point<D> {
        &self.location
    }
}
