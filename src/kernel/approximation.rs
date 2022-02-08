use parry3d_f64::shape::Segment;

use crate::math::Point;

/// An approximation of an edge, multiple edges, or a face
pub struct Approximation {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: Vec<Point<3>>,

    /// Segments that approximate edges
    ///
    /// Every approximation will involve edges, typically, and these are
    /// approximated by these segments. All the points of these segments will
    /// also be available in the `points` field of this struct.
    pub segments: Vec<Segment>,
}
