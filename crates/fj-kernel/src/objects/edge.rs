use std::fmt;

use crate::builder::EdgeBuilder;

use super::{Curve, Vertex};

/// An edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    curve: Curve,
    vertices: VerticesOfEdge,
}

impl Edge {
    /// Build an edge using [`EdgeBuilder`]
    pub fn build() -> EdgeBuilder {
        EdgeBuilder
    }

    /// Create a new instance
    pub fn new(curve: Curve, vertices: VerticesOfEdge) -> Self {
        Self { curve, vertices }
    }

    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    /// Access the vertices that bound the edge on the curve
    ///
    /// An edge has either two bounding vertices or none. The latter is possible
    /// if the edge's curve is continuous (i.e. connects to itself), and defines
    /// the whole edge.
    pub fn vertices(&self) -> &VerticesOfEdge {
        &self.vertices
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.vertices().0 {
            Some(vertices) => {
                let [a, b] = vertices.map(|vertex| vertex.position());
                write!(f, "edge from {:?} to {:?}", a, b)?
            }
            None => write!(f, "continuous edge")?,
        }

        write!(f, " on {:?}", self.curve().global())?;

        Ok(())
    }
}

/// The vertices that bound an edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VerticesOfEdge(Option<[Vertex; 2]>);

impl VerticesOfEdge {
    /// Construct an instance of `VerticesOfEdge` from zero or two vertices
    pub fn new(vertices: Option<[Vertex; 2]>) -> Self {
        Self(vertices)
    }

    /// Construct an instance of `VerticesOfEdge` from two vertices
    pub fn from_vertices(vertices: [Vertex; 2]) -> Self {
        Self(Some(vertices))
    }

    /// Construct an instance of `VerticesOfEdge` without vertices
    pub fn none() -> Self {
        Self(None)
    }

    /// Determine whether the other instance has the same vertices
    ///
    /// The order of vertices is ignored.
    pub fn are_same(&self, other: &Self) -> bool {
        if let Some([a, b]) = self.0 {
            if let Some(other) = other.0 {
                return [a, b] == other || [b, a] == other;
            }
        }

        false
    }

    /// Access the vertices
    pub fn get(&self) -> Option<[&Vertex; 2]> {
        self.0.as_ref().map(|vertices| {
            // Can be cleaned up once `each_ref` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
            let [a, b] = vertices;
            [a, b]
        })
    }

    /// Access the two vertices
    ///
    /// # Panics
    ///
    /// Panics, if the edge has no vertices.
    pub fn expect_vertices(self) -> [Vertex; 2] {
        self.0.expect("Expected edge to have vertices")
    }

    /// Iterate over the vertices
    pub fn iter(&self) -> impl Iterator<Item = &Vertex> {
        self.0.iter().flatten()
    }

    /// Reverse the order of vertices
    ///
    /// Makes sure that the local coordinates are still correct.
    pub fn reverse(self) -> Self {
        Self(self.0.map(|[a, b]| {
            [
                Vertex::new(-b.position(), *b.global()),
                Vertex::new(-a.position(), *a.global()),
            ]
        }))
    }

    /// Map each vertex using the provided function
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnMut(Vertex) -> Vertex,
    {
        Self(self.convert(f))
    }

    /// Convert each vertex using the provided function
    pub fn convert<F, T>(self, f: F) -> Option<[T; 2]>
    where
        F: FnMut(Vertex) -> T,
    {
        self.0.map(|vertices| vertices.map(f))
    }

    /// Convert each vertex using the provided fallible function
    pub fn try_convert<F, T, E>(self, f: F) -> Result<Option<[T; 2]>, E>
    where
        F: FnMut(Vertex) -> Result<T, E>,
    {
        // Can be cleaned up using `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices: Option<[Result<_, E>; 2]> = self.convert(f);
        let vertices = match vertices {
            Some([a, b]) => Some([a?, b?]),
            None => None,
        };

        Ok(vertices)
    }
}
