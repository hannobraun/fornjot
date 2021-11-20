use std::f64::consts::PI;

use crate::math::{Point, Vector};

/// An edge of a shape
///
/// See [`Shape::edges`].
pub struct Edge {
    /// The path that defines the edge
    pub path: Path,
}

impl Edge {
    /// Create an arc
    pub fn arc(radius: f64) -> Self {
        Self {
            path: Path::Arc { radius },
        }
    }

    /// Create a line segment
    pub fn line_segment(start: Point, end: Point) -> Self {
        Self {
            path: Path::LineSegment { start, end },
        }
    }

    /// Compute vertices to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edge.
    pub fn vertices(&self, tolerance: f64) -> Vec<Point> {
        match &self.path {
            Path::Arc { radius } => {
                let angle_to_point = |angle: f64| {
                    let (sin, cos) = angle.sin_cos();

                    let x = cos * radius;
                    let y = sin * radius;

                    [x, y, 0.].into()
                };

                // To approximate the circle, we use a regular polygon for which
                // the circle is the circumscribed circle. The `tolerance`
                // parameter is the maximum allowed distance between the polygon
                // and the circle. This is the same as the difference between
                // the circumscribed circle and the incircle.
                //
                // Let's figure which regular polygon we need to use, by just
                // trying out some of them until we find one whose maximum error
                // is less than or equal to the tolerance.
                let mut n = 3;
                loop {
                    let incircle_radius = radius * (PI / n as f64).cos();
                    let maximum_error = radius - incircle_radius;

                    if maximum_error <= tolerance {
                        break;
                    }

                    n += 1;
                }

                let mut vertices = Vec::new();

                let first_vertex = angle_to_point(0.0);
                vertices.push(first_vertex);

                for i in 1..n {
                    let angle = 2. * PI / n as f64 * i as f64;
                    vertices.push(angle_to_point(angle));
                }

                // Connect the circle's to itself.
                vertices.push(first_vertex);

                vertices
            }
            Path::LineSegment { start, end } => vec![*start, *end],
            Path::Approximated(vertices) => vertices.clone(),
        }
    }
}

/// A path
pub enum Path {
    /// The edge is an arc
    ///
    /// The naming here is a bit ambitious, as actual arcs aren't supported yet,
    /// only full circles.
    Arc {
        /// The radius of the arc
        radius: f64,
    },

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
