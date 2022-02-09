use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    kernel::{
        approximation::Approximation,
        geometry::{Circle, Curve},
    },
    math::Point,
};

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

    /// Compute an approximation of the edges
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edges.
    pub fn approx(&self, tolerance: f64) -> Approximation {
        let mut points = Vec::new();
        let mut segments = Vec::new();

        for cycle in &self.cycles {
            let approx = Approximation::for_cycle(cycle, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        Approximation { points, segments }
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
    pub vertices: Option<[(); 2]>,

    /// Indicates whether the curve's direction is reversed
    ///
    /// Once this struct keeps track of the vertices that bound the edge, this
    /// field can probably be made redundant. The order of the bounding points
    /// will simply define the direction of the curve.
    pub reverse: bool,
}

impl Edge {
    /// Construct an edge
    pub fn new(curve: Curve) -> Self {
        Self {
            curve,
            vertices: Some([(), ()]),
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
                radius: vector![radius, 0., 0.],
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
