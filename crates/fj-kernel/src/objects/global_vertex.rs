use std::hash::Hash;

use fj_math::Point;

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Validation
///
/// Vertices must be unique within a shape, meaning an identical vertex must not
/// exist in the same shape. In the context of vertex uniqueness, points that
/// are close to each other are considered identical. The minimum distance
/// between distinct vertices can be configured using
/// [`Shape::with_minimum_distance`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalVertex {
    position: Point<3>,
}

impl GlobalVertex {
    /// Construct a `Vertex` from a point
    pub fn from_position(position: impl Into<Point<3>>) -> Self {
        let position = position.into();
        Self { position }
    }

    /// The position of the vertex
    pub fn position(&self) -> Point<3> {
        self.position
    }
}
