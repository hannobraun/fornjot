use crate::math::{Point, Vector};

/// Access the edges of a shape
pub trait Edges {
    /// Compute vertices to approximate the shape's edges
    ///
    /// Returns a `Vec` that contains a `Vec<Point>` for each edge of the shape.
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edges of the shape.
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>>;

    /// Compute line segments to approximate the shape's edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    ///
    /// This method presents a weird API right now, as it just returns all the
    /// segments, not distinguishing which edge they approximate. This design is
    /// simple and in line with current use cases, but not expected to last.
    fn edge_segments(&self, tolerance: f64) -> Vec<Segment> {
        let mut segments = Vec::new();
        let edges = self.edge_vertices(tolerance);

        for mut vertices in edges {
            // We're about to convert these vertices into line segments, and we
            // need a connection from the last to the first.
            match vertices.first() {
                Some(&vertex) => vertices.push(vertex),
                None => {
                    // If there is not first vertex, there are no vertices. If
                    // there are no vertices, there are no segments.
                    return segments;
                }
            }

            for segment in vertices.windows(2) {
                let v0 = segment[0];
                let v1 = segment[1];

                segments.push([v0, v1].into());
            }
        }

        segments
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
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>> {
        match self {
            Self::Shape2d(shape) => shape.edge_vertices(tolerance),
            Self::Shape3d(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f64) -> Vec<Segment> {
        match self {
            Self::Shape2d(shape) => shape.edge_segments(tolerance),
            Self::Shape3d(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Shape2d {
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>> {
        match self {
            Self::Circle(shape) => shape.edge_vertices(tolerance),
            Self::Difference(shape) => shape.edge_vertices(tolerance),
            Self::Square(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f64) -> Vec<Segment> {
        match self {
            Self::Circle(shape) => shape.edge_segments(tolerance),
            Self::Difference(shape) => shape.edge_segments(tolerance),
            Self::Square(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Shape3d {
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>> {
        match self {
            Self::Sweep(shape) => shape.edge_vertices(tolerance),
        }
    }

    fn edge_segments(&self, tolerance: f64) -> Vec<Segment> {
        match self {
            Self::Sweep(shape) => shape.edge_segments(tolerance),
        }
    }
}

impl Edges for fj::Sweep {
    fn edge_vertices(&self, _tolerance: f64) -> Vec<Vec<Point>> {
        // TASK: Implement.
        todo!()
    }
}
