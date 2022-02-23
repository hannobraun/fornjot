use crate::{
    kernel::geometry::{Circle, Curve},
    math::{Point, Transform, Vector},
};

use super::vertices::Vertex;

/// The edges of a shape
#[derive(Clone)]
pub struct Edges {
    /// The cycles that the edges of the shape form
    ///
    /// Code reading this field generally assumes that cycles do not overlap.
    /// This precondition is currently not checked, and must be upheld by all
    /// code writing to this field.
    pub cycles: Vec<Cycle>,
}

impl Edges {
    /// Construct a new instance of `Edges`, with a single cycle
    pub fn single_cycle(edges: impl IntoIterator<Item = Edge>) -> Self {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        Self {
            cycles: vec![cycle],
        }
    }

    /// Transform the edges
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        for cycle in &mut self.cycles {
            for edge in &mut cycle.edges {
                *edge = edge.clone().transform(transform);
            }
        }

        self
    }
}

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone)]
pub struct Cycle {
    pub edges: Vec<Edge>,
}

/// An edge of a shape
#[derive(Clone, Debug)]
pub struct Edge {
    /// The curve that defines the edge's geometry
    ///
    /// The edge is a segment of the curve that is bounded by two vertices.
    pub curve: Curve,

    /// The vertices that bound this edge on the curve, in curve coordinates
    ///
    /// If there are no such vertices, that means the edge is connected to
    /// itself (like a full circle, for example).
    pub vertices: Option<[Vertex<1>; 2]>,
}

impl Edge {
    /// Construct an edge
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    pub fn new(curve: Curve, vertices: Option<[Vertex<3>; 2]>) -> Self {
        let vertices = vertices
            .map(|vertices| vertices.map(|vertex| vertex.to_1d(&curve)));

        Self { curve, vertices }
    }

    /// Create a circle
    pub fn circle(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle {
                center: Point::origin(),
                radius: Vector::from([radius, 0.]),
            }),
            vertices: None,
        }
    }

    /// Create a transformed edge
    ///
    /// This method constructs new instances of [`Vertex`], by calling
    /// [`Vertex::transform`].
    ///
    /// You **MUST NOT** use this method to construct a new instance of `Vertex`
    /// that represents an already existing vertex. See documentation of
    /// [`Vertex`] for more information.
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        self.curve = self.curve.transform(transform);
        self.vertices = self
            .vertices
            .map(|vertices| vertices.map(|vertex| vertex.transform(transform)));

        self
    }
}
