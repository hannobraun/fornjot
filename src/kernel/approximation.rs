use parry3d_f64::shape::Segment;

use crate::math::Point;

/// An approximation of one or more edges
pub struct Approx {
    pub vertices: Vec<Point<3>>,
    pub segments: Vec<Segment>,
}
