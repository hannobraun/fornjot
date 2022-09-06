use std::fmt;

use crate::{algorithms::reverse::Reverse, builder::EdgeBuilder};

use super::{Curve, GlobalCurve, GlobalVertex, Surface, Vertex};

/// An edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    curve: Curve,
    vertices: VerticesOfEdge<Vertex>,
    global: GlobalEdge,
}

impl Edge {
    /// Build an edge using [`EdgeBuilder`]
    pub fn build(surface: Surface) -> EdgeBuilder {
        EdgeBuilder::new(surface)
    }

    /// Create a new instance of `Edge`
    ///
    /// If you only have a curve and the edge vertices, please check out
    /// [`Edge::from_curve_and_vertices`], which is a convenience wrapper around
    /// this method, which creates an instance of [`GlobalEdge`].
    ///
    /// # Panics
    ///
    /// Panics, if the provided [`GlobalEdge`] instance doesn't refer to the
    /// same [`GlobalCurve`] and [`GlobalVertex`] instances that the other
    /// objects that are passed refer to.
    pub fn new(
        curve: Curve,
        vertices: VerticesOfEdge<Vertex>,
        global: GlobalEdge,
    ) -> Self {
        assert_eq!(curve.global_form(), global.curve());
        assert_eq!(&vertices.to_global(), global.vertices());

        // Make sure that the edge vertices are not coincident on the curve. If
        // they were, the edge would have no length, and not be valid.
        //
        // It is perfectly fine for global forms of the the vertices to be
        // coincident (in 3D space). That would just mean, that ends of the edge
        // connect to each other.
        if let Some([a, b]) = vertices.get() {
            assert_ne!(
                a.position(),
                b.position(),
                "Vertices of an edge must not be coincident on curve"
            );
        }

        Self {
            curve,
            vertices,
            global,
        }
    }

    /// Create a new instance of `Edge` from a curve and vertices
    ///
    /// The [`GlobalEdge`] instance is created from the provided curve and
    /// vertices. Please refer to [`Edge::new`], if you already have a
    /// [`GlobalEdge`] instance that you can provide.
    pub fn from_curve_and_vertices(
        curve: Curve,
        vertices: VerticesOfEdge<Vertex>,
    ) -> Self {
        let global =
            GlobalEdge::new(*curve.global_form(), vertices.to_global());
        Self::new(curve, vertices, global)
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
    pub fn global_form(&self) -> &GlobalEdge {
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

        write!(f, " on {:?}", self.curve().global_form())?;

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
                Vertex::new(
                    -b.position(),
                    b.curve().reverse(),
                    *b.surface_form(),
                    *b.global_form(),
                ),
                Vertex::new(
                    -a.position(),
                    a.curve().reverse(),
                    *a.surface_form(),
                    *a.global_form(),
                ),
            ]
        }))
    }

    /// Convert this instance into its global variant
    pub fn to_global(&self) -> VerticesOfEdge<GlobalVertex> {
        VerticesOfEdge(self.convert(|vertex| *vertex.global_form()))
    }
}
