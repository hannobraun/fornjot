use std::f32::consts::PI;

use crate::{
    geometry::vertices::Vertices as _,
    math::{Point, Vector},
};

/// Access the edges of a shape
pub trait Edges {
    /// Compute line segments to approximate the shape's edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn segments(&self, tolerance: f32) -> Segments;
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
    fn segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Shape2d(shape) => shape.segments(tolerance),
            Self::Shape3d(shape) => shape.segments(tolerance),
        }
    }
}

impl Edges for fj::Shape2d {
    fn segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Circle(shape) => shape.segments(tolerance),
            Self::Difference(shape) => shape.segments(tolerance),
            Self::Square(shape) => shape.segments(tolerance),
        }
    }
}

impl Edges for fj::Shape3d {
    fn segments(&self, tolerance: f32) -> Segments {
        match self {
            Self::Sweep(shape) => shape.segments(tolerance),
        }
    }
}

impl Edges for fj::Circle {
    fn segments(&self, tolerance: f32) -> Segments {
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

        let mut vertices = Vec::new();
        for i in 0..n {
            let angle = 2. * PI / n as f32 * i as f32;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            vertices.push(Point::new(x, y, 0.0));
        }

        // We're about to convert these vertices into line segments, and we need
        // a connection from the last to the first.
        //
        // The indexing operation can't panic, as we've initialized `n` with `3`
        // above, hence there must be at least 3 vertices in `vertices`.
        vertices.push(vertices[0]);

        let mut segments = Segments::new();
        for segment in vertices.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            segments.push([v0, v1]);
        }

        segments
    }
}

impl Edges for fj::Difference {
    fn segments(&self, _tolerance: f32) -> Segments {
        // TASK: Implement.
        todo!()
    }
}

impl Edges for fj::Square {
    fn segments(&self, _: f32) -> Segments {
        let mut segments = Segments::new();

        let v = self.vertices();

        segments.push([v[0], v[1]]);
        segments.push([v[1], v[2]]);
        segments.push([v[2], v[3]]);
        segments.push([v[3], v[0]]);

        segments
    }
}

impl Edges for fj::Sweep {
    fn segments(&self, _tolerance: f32) -> Segments {
        // TASK: Implement.
        todo!()
    }
}
