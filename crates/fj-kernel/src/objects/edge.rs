use std::fmt;

use crate::{algorithms::reverse::Reverse, builder::EdgeBuilder};

use super::{Curve, GlobalCurve, GlobalVertex, Surface, Vertex};

/// An edge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    curve: Curve,
    vertices: [Vertex; 2],
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
        vertices: [Vertex; 2],
        global: GlobalEdge,
    ) -> Self {
        assert_eq!(curve.global_form(), global.curve());
        assert_eq!(
            &vertices.map(|vertex| *vertex.global_form()),
            global.vertices()
        );

        // Make sure that the edge vertices are not coincident on the curve. If
        // they were, the edge would have no length, and not be valid.
        //
        // It is perfectly fine for global forms of the the vertices to be
        // coincident (in 3D space). That would just mean, that ends of the edge
        // connect to each other.
        let [a, b] = vertices;
        assert_ne!(
            a.position(),
            b.position(),
            "Vertices of an edge must not be coincident on curve"
        );

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
        vertices: [Vertex; 2],
    ) -> Self {
        let global = GlobalEdge::new(
            *curve.global_form(),
            vertices.map(|vertex| *vertex.global_form()),
        );
        Self::new(curve, vertices, global)
    }

    /// Reverse the edge, including the curve
    ///
    /// # Implementation Note
    ///
    /// It would be much nicer to just reverse the edge normally everywhere, but
    /// we can't do that, until #695 is addressed:
    /// <https://github.com/hannobraun/Fornjot/issues/695>
    pub fn reverse_including_curve(self) -> Self {
        let vertices = {
            let [a, b] = self.vertices;
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
        };

        Self::from_curve_and_vertices(self.curve().reverse(), vertices)
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
    pub fn vertices(&self) -> &[Vertex; 2] {
        &self.vertices
    }

    /// Access the global form of this edge
    pub fn global_form(&self) -> &GlobalEdge {
        &self.global
    }
}

impl fmt::Display for Edge {
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
