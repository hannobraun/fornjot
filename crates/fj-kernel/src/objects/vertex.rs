use std::hash::Hash;

use fj_math::Point;

use crate::{builder::VertexBuilder, shape::Shape};

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
pub struct Vertex {
    /// The point that defines the location of the vertex
    pub point: Point<3>,
}

impl Vertex {
    /// Construct a `Vertex` from a point
    pub fn from_point(point: impl Into<Point<3>>) -> Self {
        let point = point.into();
        Self { point }
    }

    /// Build a vertex using the [`VertexBuilder`] API
    pub fn builder(shape: &mut Shape) -> VertexBuilder {
        VertexBuilder::new(shape)
    }
}
