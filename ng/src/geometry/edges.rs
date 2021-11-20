use std::f32::consts::PI;

use crate::{
    geometry::vertices::Vertices as _,
    math::{Point, Vector},
};

/// Access the edges of a shape
pub trait Edges {
    /// Compute vertices to approximate the shape's edges
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn edge_vertices(&self, tolerance: f32) -> EdgeVertices;

    /// Compute line segments to approximate the shape's edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn edge_segments(&self, tolerance: f32) -> Segments {
        let mut segments = Segments::new();
        let mut vertices = self.edge_vertices(tolerance);

        // We're about to convert these vertices into line segments, and we need
        // a connection from the last to the first.
        match vertices.0.first() {
            Some(&vertex) => vertices.push(vertex),
            None => {
                // If there is not first vertex, there are no vertices. If there
                // are no vertices, there are no segments.
                return segments;
            }
        }

        for segment in vertices.0.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            segments.push([v0, v1]);
        }

        segments
    }
}

/// Vertices that approximate a shape's edges
#[derive(Debug)]
pub struct EdgeVertices(pub Vec<Point>);

impl EdgeVertices {
    /// Create a new instance of `EdgeVertices`
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a vertex
    pub fn push(&mut self, vertex: impl Into<Point>) {
        self.0.push(vertex.into())
    }
}

/// Line segments that approximate a shape's edges
#[derive(Debug)]
pub struct Segments(pub Vec<Segment>);

impl Segments {
    /// Create a new instance of `Segments`
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a segment
    pub fn push(&mut self, segment: impl Into<Segment>) {
        self.0.push(segment.into())
    }
}

/// A line segment
#[derive(Debug)]
pub struct Segment(pub [Point; 2]);

impl Segment {
    /// Translate the segment
    ///
    /// Translate all segment vertices by the given vector.
    pub fn translate(self, vector: Vector) -> Self {
        let vertices = self.0.map(|vertex| vertex + vector);
        Self(vertices)
    }
}

impl From<[Point; 2]> for Segment {
    fn from(vertices: [Point; 2]) -> Self {
        Self(vertices)
    }
}

impl Edges for fj::Shape {
    fn edge_vertices(&self, tolerance: f32) -> EdgeVertices {
        match self {
            Self::Shape2d(shape) => shape.edge_vertices(tolerance),
            Self::Shape3d(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Shape2d(shape) => shape.edge_segments(tolerance),
            Self::Shape3d(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Shape2d {
    fn edge_vertices(&self, tolerance: f32) -> EdgeVertices {
        match self {
            Self::Circle(shape) => shape.edge_vertices(tolerance),
            Self::Difference(shape) => shape.edge_vertices(tolerance),
            Self::Square(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Circle(shape) => shape.edge_segments(tolerance),
            Self::Difference(shape) => shape.edge_segments(tolerance),
            Self::Square(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Shape3d {
    fn edge_vertices(&self, tolerance: f32) -> EdgeVertices {
        match self {
            Self::Sweep(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Sweep(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Circle {
    fn edge_vertices(&self, tolerance: f32) -> EdgeVertices {
        // To approximate the circle, we use a regular polygon for which the
        // circle is the circumscribed circle. The `tolerance` parameter is the
        // maximum allowed distance between the polygon and the circle. This is
        // the same as the difference between the circumscribed circle and the
        // incircle.
        //
        // Let's figure which regular polygon we need to use, by just trying out
        // some of them until we find one whose maximum error is less than or
        // equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius * (PI / n as f32).cos();
            let maximum_error = self.radius - incircle_radius;

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut vertices = EdgeVertices::new();
        for i in 0..n {
            let angle = 2. * PI / n as f32 * i as f32;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            vertices.push([x, y, 0.]);
        }

        vertices
    }
}

impl Edges for fj::Difference {
    fn edge_vertices(&self, _tolerance: f32) -> EdgeVertices {
        // TASK: Implement.
        todo!()
    }
}

impl Edges for fj::Square {
    fn edge_vertices(&self, _: f32) -> EdgeVertices {
        EdgeVertices(self.vertices())
    }
}

impl Edges for fj::Sweep {
    fn edge_vertices(&self, _tolerance: f32) -> EdgeVertices {
        // TASK: Implement.
        todo!()
    }
}
