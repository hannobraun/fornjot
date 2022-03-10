use crate::{kernel::shape::handle::Handle, math::Point};

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {
    pub point: Handle<Point<3>>,
}

impl Vertex {
    /// Access the point that the vertex refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn point(&self) -> Point<3> {
        *self.point.get()
    }
}
