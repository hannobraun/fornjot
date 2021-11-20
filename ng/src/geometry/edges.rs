use crate::math::{Point, Vector};

/// An edge of a shape
///
/// See [`Shape::edges`].
pub enum Edge {
    /// The edge is a line segment
    LineSegment {
        /// The start of the line segment
        start: Point,

        /// The end of the line segment
        end: Point,
    },

    /// The edge is approximated through vertices
    ///
    /// This variant only exists temporarily while a refactoring is going on.
    Approximated(Vec<Point>),
}

impl Edge {
    /// Create a line segment
    pub fn line_segment(start: Point, end: Point) -> Self {
        Self::LineSegment { start, end }
    }

    /// Access the vertices
    pub fn vertices(&self, _tolerance: f64) -> Vec<Point> {
        match self {
            Self::LineSegment { start, end } => vec![*start, *end],
            Self::Approximated(vertices) => vertices.clone(),
        }
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
