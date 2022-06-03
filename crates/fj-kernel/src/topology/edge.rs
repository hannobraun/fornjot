use std::fmt;

use fj_math::Point;

use crate::{
    geometry::Curve,
    shape::{Handle, LocalForm, Shape},
};

use super::{EdgeBuilder, Vertex};

/// An edge of a shape
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// An edge that is part of a [`Shape`] must be structurally sound. That means
/// the curve and any vertices that it refers to, must be part of the same
/// shape.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge<const D: usize> {
    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub curve: LocalForm<Curve<D>, Curve<3>>,

    /// Access the vertices that bound the edge on the curve
    ///
    /// If there are no such vertices, that means that both the curve and the
    /// edge are continuous (i.e. connected to themselves).
    pub vertices: VerticesOfEdge,
}

impl Edge<3> {
    /// Construct an instance of `Edge`
    pub fn new(curve: Handle<Curve<3>>, vertices: VerticesOfEdge) -> Self {
        let curve = LocalForm::canonical_only(curve);
        Self { curve, vertices }
    }

    /// Build an edge using the [`EdgeBuilder`] API
    pub fn builder(shape: &mut Shape) -> EdgeBuilder {
        EdgeBuilder::new(shape)
    }
}

impl<const D: usize> Edge<D> {
    /// Access the curve that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn curve(&self) -> Curve<3> {
        self.curve.canonical().get()
    }

    /// Access the vertices that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn vertices(&self) -> Option<[Vertex; 2]> {
        self.vertices
            .0
            .as_ref()
            .map(|[a, b]| [a.canonical().get(), b.canonical().get()])
    }
}

impl<const D: usize> fmt::Display for Edge<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.vertices() {
            Some(vertices) => {
                let [a, b] = vertices.map(|vertex| vertex.point());
                write!(f, "edge from {:?} to {:?}", a, b)?
            }
            None => write!(f, "continuous edge")?,
        }

        write!(f, " on {}", self.curve())?;

        Ok(())
    }
}

/// The vertices that bound an edge
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VerticesOfEdge(Option<[LocalForm<Point<1>, Vertex>; 2]>);

impl VerticesOfEdge {
    /// Construct an instance of `VerticesOfEdge` from zero or two vertices
    pub fn new(vertices: Option<[LocalForm<Point<1>, Vertex>; 2]>) -> Self {
        Self(vertices)
    }

    /// Construct an instance of `VerticesOfEdge` from two vertices
    pub fn from_vertices(vertices: [LocalForm<Point<1>, Vertex>; 2]) -> Self {
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
        if let Some([a, b]) = &self.0 {
            let [a, b] = [a.canonical(), b.canonical()];

            if let Some(other) = &other.0 {
                let other = {
                    let [a, b] = other;
                    [a.canonical(), b.canonical()]
                };

                return [a.clone(), b.clone()] == other || [b, a] == other;
            }
        }

        false
    }

    /// Access the two vertices
    ///
    /// # Panics
    ///
    /// Panics, if the edge has no vertices.
    pub fn expect_vertices(self) -> [LocalForm<Point<1>, Vertex>; 2] {
        self.0.expect("Expected edge to have vertices")
    }

    /// Iterate over the vertices, if any
    pub fn iter(&self) -> impl Iterator<Item = &LocalForm<Point<1>, Vertex>> {
        self.0.iter().flatten()
    }

    /// Reverse the order of vertices
    ///
    /// Makes sure that the local coordinates are still correct.
    pub fn reverse(self) -> Self {
        Self(self.0.map(|[a, b]| {
            [
                LocalForm::new(-(*b.local()), b.canonical()),
                LocalForm::new(-(*a.local()), a.canonical()),
            ]
        }))
    }

    /// Map each vertex using the provided function
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnMut(LocalForm<Point<1>, Vertex>) -> LocalForm<Point<1>, Vertex>,
    {
        Self(self.convert(f))
    }

    /// Convert each vertex using the provided function
    pub fn convert<F, T>(self, f: F) -> Option<[T; 2]>
    where
        F: FnMut(LocalForm<Point<1>, Vertex>) -> T,
    {
        self.0.map(|vertices| vertices.map(f))
    }

    /// Convert each vertex using the provided fallible function
    pub fn try_convert<F, T, E>(self, f: F) -> Result<Option<[T; 2]>, E>
    where
        F: FnMut(LocalForm<Point<1>, Vertex>) -> Result<T, E>,
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
