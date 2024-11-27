//! # The primitive operations
//!
//! Future experiments may support user-defined operations on top of those, but
//! for now this is all there is.

use std::fmt;

use crate::math::Point;

use super::Operation;

/// # A vertex
///
/// This is the most basic operation, which creates a single vertex, represented
/// as a point in 3D space.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Vertex {
    /// # The point that represents the vertex
    pub point: Point,
}

impl<P> From<P> for Vertex
where
    P: Into<Point>,
{
    fn from(point: P) -> Self {
        Self {
            point: point.into(),
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [x, y, z] = self.point.coords.components.map(|s| s.value());
        write!(f, "vertex {x:.2}, {y:.2}, {z:.2}")
    }
}

impl Operation for Vertex {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.push(*self);
    }

    fn triangles(&self, _: &mut Vec<Triangle>) {}
}

/// # A triangle
///
/// Combines three [`Vertex`] instances into a triangle. This is still very
/// simple, but still kind of a prototype for how other non-trivial operations
/// that combine more primitives ones might later look like.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triangle {
    /// # The vertices of the triangle
    ///
    /// This embeds a copy of the vertex, which isn't great for two reasons:
    ///
    /// - It is rather space-inefficient, as multiple copies of every vertex are
    ///   bound to exist within the shape.
    /// - It loses the identity of the vertex, making it harder to highlight it
    ///   in the viewer, for example, or to run validation code that could
    ///   possibly exist later.
    ///
    /// For now, that's fine. Follow-up experiments will likely use some kind
    /// of centralized storage of operations, and handles that refer to the
    /// operations in those stores, similar to what mainline Fornjot does with
    /// its topographic objects.
    pub vertices: [Vertex; 3],
}

impl<V> From<[V; 3]> for Triangle
where
    V: Into<Vertex>,
{
    fn from(vertices: [V; 3]) -> Self {
        Self {
            vertices: vertices.map(Into::into),
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c] = self.vertices;
        write!(f, "triangle {a} - {b} - {c}")
    }
}

impl Operation for Triangle {
    fn vertices(&self, _: &mut Vec<Vertex>) {}

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.push(*self)
    }
}
