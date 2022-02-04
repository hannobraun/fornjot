use nalgebra::vector;
use parry3d_f64::{math::Isometry, shape::Segment as Segment3};

use crate::{
    kernel::geometry::{Circle, Curve},
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
                edge.curve = edge.curve.transform(transform);
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
    pub fn approx(&self, tolerance: f64) -> Approx {
        let mut vertices = Vec::new();
        let mut segments = Vec::new();

        for cycle in &self.cycles {
            let approx = cycle.approx(tolerance);

            vertices.extend(approx.vertices);
            segments.extend(approx.segments);
        }

        Approx { vertices, segments }
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
    pub fn approx(&self, tolerance: f64) -> Approx {
        let mut vertices = Vec::new();
        for edge in &self.edges {
            vertices.extend(edge.approx_vertices(tolerance));
        }

        // As this is a cycle, the last vertex of an edge could be identical to
        // the first vertex of the next. Let's remove those duplicates.
        vertices.dedup();

        let mut segments = Vec::new();
        self.approx_segments(tolerance, &mut segments);

        Approx { vertices, segments }
    }

    /// Compute segments to approximate the edges of this cycle
    ///
    /// `tolerance` defines how far these segments are allowed to deviate from
    /// the actual edges of the shape.
    ///
    /// No assumptions must be made about already existing contents of `out`, as
    /// this method might modify them.
    pub fn approx_segments(&self, tolerance: f64, out: &mut Vec<Segment3>) {
        for edge in &self.edges {
            edge.approx_segments(tolerance, out);
        }
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

    /// Compute vertices to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edge.
    pub fn approx_vertices(&self, tolerance: f64) -> Vec<Point<3>> {
        // This method doesn't follow the style of the other methods that return
        // approximate vertices, allocating its output `Vec` itself, instead of
        // using one passed into it as a mutable reference.
        //
        // I initially intended to convert all these methods to the new style
        // (i.e. the pass `&mut Vec` style), until I hit this one. The problem
        // here is the `reverse` below. Doing that on a passed in `Vec` would
        // be disruptive to callers and keeping track of the slice to call the
        // `reverse` on would be additional complexity.
        //
        // I don't know what to do about that, but I think leaving things as
        // they are and writing this comment to explain that is a good enough
        // solution.

        let mut out = Vec::new();
        self.curve.approx_vertices(tolerance, &mut out);

        if self.reverse {
            out.reverse()
        }

        out
    }

    /// Compute segments to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// segments are allowed to deviate from the actual edge.
    pub fn approx_segments(&self, tolerance: f64, out: &mut Vec<Segment3>) {
        let mut vertices = self.approx_vertices(tolerance);

        if self.vertices.is_none() {
            // The edge has no vertices, which means it connects to itself. We
            // need to reflect that in the approximation.

            if let Some(&vertex) = vertices.first() {
                vertices.push(vertex);
            }
        }

        for segment in vertices.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            out.push([v0, v1].into());
        }
    }
}

/// An approximation of one or more edges
pub struct Approx {
    pub vertices: Vec<Point<3>>,
    pub segments: Vec<Segment3>,
}
