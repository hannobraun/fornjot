use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    kernel::geometry::{Circle, Curve},
    math::Point,
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
    pub fn transform(mut self, transform: &Isometry<f64>) -> Self {
        for cycle in &mut self.cycles {
            for edge in &mut cycle.edges {
                edge.curve = edge.curve.clone().transform(transform);
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
    ///
    /// This field is a placeholder. Eventually, there will be actual vertices
    /// here. For now, this field just tracks whether there are such bounding
    /// vertices or not. If there are, they are implicitly assumed to be the
    /// points with the curve coordinates `0` and `1`.
    ///
    /// # Implementation note
    ///
    /// Once we add vertices here, we'll need to update the approximation code
    /// to support that. Down on the curve level, the approximation method must
    /// not return approximations of the vertices themselves, as those would
    /// prevent duplicate vertices from being detected reliably, hence
    /// compromising the the correctness of the whole approximation.
    ///
    /// To prevent this, curves must only return approximated points _between_
    /// the vertices. The following might be a good method signature to achieve
    /// that:
    /// ``` rust
    /// fn approximate_between(&self, vertices: &Option<[Vertex; 2]>)
    ///     -> Vev<Point<3>>
    /// {
    ///     // ...
    /// }
    /// ```
    ///
    /// When considering how to implement such a method, it becomes obvious that
    /// passing 3D vertices would be kind of a pain. Not only would those have
    /// to be converted into 1D curve coordinates to be useful, making the
    /// implementation cumbersome, it would also make the method fallible,
    /// exposing the inherent error-proneness of representing points that bound
    /// a vertex on a curve in 3D.
    ///
    /// The logical conclusion is that vertices here should be represented in 1D
    /// curve coordinates, only being converted into 3D points for the
    /// approximation.
    pub vertices: Option<[Vertex<1>; 2]>,

    /// Indicates whether the curve's direction is reversed
    ///
    /// Once this struct keeps track of the vertices that bound the edge, this
    /// field can probably be made redundant. The order of the bounding points
    /// will simply define the direction of the curve.
    pub reverse: bool,
}

impl Edge {
    /// Construct an edge
    pub fn new(curve: Curve, vertices: Option<[Vertex<3>; 2]>) -> Self {
        let vertices = vertices
            .map(|vertices| vertices.map(|vertex| vertex.to_1d(&curve)));

        Self {
            curve,
            vertices,
            reverse: false,
        }
    }

    /// Create an arc
    ///
    /// So far, the name of this method is a bit ambitious, as only full circles
    /// are supported.
    pub fn arc(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle {
                center: Point::origin(),
                radius: vector![radius, 0.],
            }),
            vertices: None,
            reverse: false,
        }
    }

    /// Reverse the edge
    pub fn reverse(&mut self) {
        self.reverse = !self.reverse;
    }
}
