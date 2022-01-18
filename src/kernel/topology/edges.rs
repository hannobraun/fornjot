use nalgebra::{point, vector};
use parry2d_f64::shape::Segment as Segment2;
use parry3d_f64::{math::Isometry, shape::Segment as Segment3};

use crate::{
    kernel::geometry::{Circle, Curve, Line, Surface},
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
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        for cycle in &mut self.cycles {
            for edge in &mut cycle.edges {
                edge.curve.transform(transform);
            }
        }
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
    ///
    /// To achieve this, some infrastructure improvements would have to be made:
    ///
    /// 1. Edges would need to be identified by a unique ID, and the
    ///    approximations in the data structure returned here would need to be
    ///    associated with that ID, to allow for retrieval.
    /// 2. Edges would need to be approximated in curve coordinates.
    ///    - As of this writing, they are computed in model coordinates, then
    ///      converted to surface coordinates, which is just a nuisance.
    ///    - Directly computing them in surface coordinates would be an
    ///      improvement, but would obviously not allow for the reuse of results
    ///      that is imagined here.
    ///    - Hence computing the approximation in curve coordinates, then
    ///      converting it to surface coordinates for the triangulation, seems
    ///      to be the prudent choice.
    ///
    /// In addition, it should be noted that edge approximations in curve
    /// coordinates could be reused even for different edges, as long as those
    /// edges are distance-preserving transformations (isometries) of the
    /// approximated edge.
    ///
    /// To realize all this, it is probably best to have a single, big
    /// `EdgeApprox` struct with an `approximate_edge` method that takes a
    /// `&Edges` and returns the approximation for the edge. Either directly
    /// from the cache, or computing it first.
    pub fn approx(&self, tolerance: f64, surface: &Surface) -> Approx {
        let mut vertices = Vec::new();
        for cycle in &self.cycles {
            cycle.approx_vertices(tolerance, &mut vertices);
        }

        // This needlessly calls `self.approx_vertices` again, internally. The
        // vertices are already computed, so they can just be removed.
        let mut segments = Vec::new();
        self.approx_segments(tolerance, &mut segments);

        let vertices = vertices
            .into_iter()
            .map(|vertex| {
                // Can't panic, unless the approximation wrongfully generates
                // points that are not in the surface.
                surface.point_model_to_surface(vertex).unwrap()
            })
            .collect();
        let segments = segments
            .into_iter()
            .map(|Segment3 { a, b }| {
                // Can't panic, unless the approximation wrongfully generates
                // points that are not in the surface.
                let a = surface.point_model_to_surface(a).unwrap();
                let b = surface.point_model_to_surface(b).unwrap();

                Segment2 { a, b }
            })
            .collect();

        Approx { vertices, segments }
    }

    /// Compute line segments to approximate the edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    pub fn approx_segments(&self, tolerance: f64, out: &mut Vec<Segment3>) {
        for cycle in &self.cycles {
            cycle.approx_segments(tolerance, out);
        }
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
    /// Compute vertices to approximate the edges of this cycle
    ///
    /// `tolerance` defines how far these vertices are allowed to deviate from
    /// the actual edges of the shape.
    ///
    /// No assumptions must be made about already existing contents of `out`, as
    /// this method might modify them.
    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        for edge in &self.edges {
            out.extend(edge.approx_vertices(tolerance));
        }

        // As this is a cycle, the last vertex of an edge could be identical to
        // the first vertex of the next. Let's remove those duplicates.
        out.dedup();
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
    pub vertices: Option<[Point<1>; 2]>,

    /// Indicates whether the curve's direction is reversed
    pub reverse: bool,
}

impl Edge {
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

    /// Create a line segment
    pub fn line_segment(start: Point<3>, end: Point<3>) -> Self {
        Self {
            curve: Curve::Line(Line {
                origin: start,
                dir: end - start,
            }),
            vertices: Some([point![0.], point![1.]]),
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
    pub vertices: Vec<Point<2>>,
    pub segments: Vec<Segment2>,
}
