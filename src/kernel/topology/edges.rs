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
    ///
    /// # Implementation note
    ///
    /// As of this writing, each edge is approximated multiple times, for the
    /// triangulation of each face that it is adjacent to. This might not be
    /// desirable, for the following reasons:
    ///
    /// 1. Efficiency: Approximating an edge once and caching the result might
    ///    realize a performance gain.
    /// 2. Correctness: It is conceivable that the same edge is approximated
    ///    differently for each of its neighboring faces, if the algorithm used
    ///    is not fully deterministic. If that were to happen, this would result
    ///    in a triangle mesh where the triangles don't connect.
    ///
    /// Only approximating an edge once, and then referring to that
    /// approximation from then on where needed, would take care of these two
    /// problems.
    pub fn approx(&self, tolerance: f64) -> Approximation {
        let mut points = Vec::new();
        let mut segments = Vec::new();

        for cycle in &self.cycles {
            let approx = cycle.approx(tolerance);

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

impl Cycle {
    /// Compute an approximation of the cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual cycle.
    pub fn approx(&self, tolerance: f64) -> Approximation {
        let mut points = Vec::new();
        let mut segments = Vec::new();

        for edge in &self.edges {
            let approx = edge.approx(tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        // As this is a cycle, the last vertex of an edge could be identical to
        // the first vertex of the next. Let's remove those duplicates.
        points.dedup();

        Approximation { points, segments }
    }
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

    /// Compute an approximation of the edge
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    pub fn approx(&self, tolerance: f64) -> Approximation {
        let mut points = Vec::new();
        self.curve.approx(tolerance, &mut points);

        if self.reverse {
            points.reverse()
        }

        let mut segment_vertices = points.clone();
        if self.vertices.is_none() {
            // The edge has no vertices, which means it connects to itself. We
            // need to reflect that in the approximation.

            if let Some(&vertex) = points.first() {
                segment_vertices.push(vertex);
            }
        }

        let mut segments = Vec::new();
        for segment in segment_vertices.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            segments.push([v0, v1].into());
        }

        Approximation { points, segments }
    }
}
