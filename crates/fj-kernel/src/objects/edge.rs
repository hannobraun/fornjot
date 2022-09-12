use std::fmt;

use crate::builder::HalfEdgeBuilder;

use super::{Curve, GlobalCurve, GlobalVertex, Surface, Vertex};

/// A half-edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdge {
    curve: Curve,
    vertices: [Vertex; 2],
    global_form: GlobalEdge,
}

impl HalfEdge {
    /// Build a half-edge using [`HalfEdgeBuilder`]
    pub fn build(surface: Surface) -> HalfEdgeBuilder {
        HalfEdgeBuilder::new(surface)
    }

    /// Create a new instance of `HalfEdge`
    ///
    /// If you only have a curve and the edge vertices, please check out
    /// [`HalfEdge::from_curve_and_vertices`], which is a convenience wrapper
    /// around this method, which creates an instance of [`GlobalEdge`].
    ///
    /// # Panics
    ///
    /// Panics, if the provided [`GlobalEdge`] instance doesn't refer to the
    /// same [`GlobalCurve`] and [`GlobalVertex`] instances that the other
    /// objects that are passed refer to.
    ///
    /// Panics, if the provided vertices are coincident on the curve. If they
    /// were, the edge would have no length, and thus not be valid. (It is
    /// perfectly fine for global forms of the the vertices to be coincident.
    /// That would just mean, that ends of the edge connect to each other.)
    pub fn new(
        curve: Curve,
        vertices: [Vertex; 2],
        global_form: GlobalEdge,
    ) -> Self {
        // Make sure `curve` and `vertices` match.
        for vertex in vertices {
            assert_eq!(
                &curve,
                vertex.curve(),
                "An edge and its vertices must be defined on the same curve"
            );
        }

        // Make sure `curve` and `vertices` match `global_form`.
        assert_eq!(curve.global_form(), global_form.curve());
        assert_eq!(
            &vertices.map(|vertex| *vertex.global_form()),
            global_form.vertices()
        );

        // Make sure that the edge vertices are not coincident on the curve.
        let [a, b] = vertices;
        assert_ne!(
            a.position(),
            b.position(),
            "Vertices of an edge must not be coincident on curve"
        );

        Self {
            curve,
            vertices,
            global_form,
        }
    }

    /// Create a new instance of `HalfEdge` from a curve and vertices
    ///
    /// The [`GlobalEdge`] instance is created from the provided curve and
    /// vertices. Please refer to [`HalfEdge::new`], if you already have a
    /// [`GlobalEdge`] instance that you can provide.
    pub fn from_curve_and_vertices(
        curve: Curve,
        vertices: [Vertex; 2],
    ) -> Self {
        let global = GlobalEdge::new(
            *curve.global_form(),
            vertices.map(|vertex| *vertex.global_form()),
        );
        Self::new(curve, vertices, global)
    }

    /// Access the curve that defines the half-edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    /// Access the vertices that bound the half-edge on the curve
    ///
    /// An edge has either two bounding vertices or none. The latter is possible
    /// if the edge's curve is continuous (i.e. connects to itself), and defines
    /// the whole edge.
    pub fn vertices(&self) -> &[Vertex; 2] {
        &self.vertices
    }

    /// Access the global form of this half-edge
    pub fn global_form(&self) -> &GlobalEdge {
        &self.global_form
    }
}

impl fmt::Display for HalfEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b] = self.vertices().map(|vertex| vertex.position());
        write!(f, "edge from {:?} to {:?}", a, b)?;
        write!(f, " on {:?}", self.curve().global_form())?;

        Ok(())
    }
}

/// An edge, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalEdge {
    curve: GlobalCurve,
    vertices: [GlobalVertex; 2],
}

impl GlobalEdge {
    /// Create a new instance
    pub fn new(curve: GlobalCurve, vertices: [GlobalVertex; 2]) -> Self {
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
    pub fn vertices(&self) -> &[GlobalVertex; 2] {
        &self.vertices
    }
}
