use std::fmt;

use crate::builder::EdgeBuilder;

use super::{Curve, GlobalCurve, GlobalVertex, Vertex};

/// An edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    curve: Curve,
    vertices: VerticesOfEdge<Vertex>,
    global: GlobalEdge,
}

impl Edge {
    /// Build an edge using [`EdgeBuilder`]
    pub fn build() -> EdgeBuilder {
        EdgeBuilder
    }

    /// Create a new instance
    pub fn new(curve: Curve, vertices: VerticesOfEdge<Vertex>) -> Self {
        let global = GlobalEdge::new(*curve.global(), vertices.to_global());

        Self {
            curve,
            vertices,
            global,
        }
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
    pub fn vertices(&self) -> &VerticesOfEdge<Vertex> {
        &self.vertices
    }

    /// Access the global form of this edge
    pub fn global(&self) -> &GlobalEdge {
        &self.global
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

/// An edge, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalEdge {
    curve: GlobalCurve,
    vertices: VerticesOfEdge<GlobalVertex>,
}

impl GlobalEdge {
    /// Create a new instance
    pub fn new(
        curve: GlobalCurve,
        vertices: VerticesOfEdge<GlobalVertex>,
    ) -> Self {
        Self { curve, vertices }
    }

    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub fn curve(&self) -> &GlobalCurve {
        &self.curve
    }

    /// Access the vertices that bound the edge on the curve
    ///
    /// An edge has either two bounding vertices or none. The latter is possible
    /// if the edge's curve is continuous (i.e. connects to itself), and defines
    /// the whole edge.
    pub fn vertices(&self) -> &VerticesOfEdge<GlobalVertex> {
        &self.vertices
    }
}

/// The vertices that bound an edge
///
/// This struct is generic over the actual vertex type used, but typically, `T`
/// will either be [`Vertex`] or [`GlobalVertex`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VerticesOfEdge<T>(Option<[T; 2]>);

impl<T> VerticesOfEdge<T> {
    /// Construct an instance of `VerticesOfEdge` from two vertices
    pub fn from_vertices(vertices: [T; 2]) -> Self {
        Self(Some(vertices))
    }

    /// Construct an instance of `VerticesOfEdge` without vertices
    pub fn none() -> Self {
        Self(None)
    }

    /// Access the vertices
    pub fn get(&self) -> Option<[&T; 2]> {
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
    pub fn get_or_panic(&self) -> [&T; 2] {
        self.get().expect("Expected edge to have vertices")
    }

    /// Iterate over the vertices
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flatten()
    }

    /// Map each vertex using the provided function
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        Self(self.convert(f))
    }

    /// Convert each vertex using the provided function
    pub fn convert<F, U>(self, f: F) -> Option<[U; 2]>
    where
        F: FnMut(T) -> U,
    {
        self.0.map(|vertices| vertices.map(f))
    }
}

impl VerticesOfEdge<Vertex> {
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

    /// Convert this instance into its global variant
    pub fn to_global(&self) -> VerticesOfEdge<GlobalVertex> {
        VerticesOfEdge(self.convert(|vertex| *vertex.global()))
    }
}
