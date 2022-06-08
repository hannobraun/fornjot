use std::hash::Hash;

use fj_math::Point;

use crate::shape::Shape;

use super::VertexBuilder;

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// A vertex that is part of a [`Shape`] must be structurally sound. That means
/// the point it refers to must be part of the same shape.
///
/// Vertices must be unique within a shape, meaning another vertex defined by
/// the same shape must not exist. In the context of vertex uniqueness, points
/// that are close to each other are considered identical. The minimum distance
/// between distinct vertices can be configured using
/// [`Shape::with_minimum_distance`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {
    /// The point that defines the location of the vertex
    pub point: Point<3>,
}

impl Vertex {
    /// Build a vertex using the [`VertexBuilder`] API
    pub fn builder(shape: &mut Shape) -> VertexBuilder {
        VertexBuilder::new(shape)
    }
}
