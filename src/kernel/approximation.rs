use parry3d_f64::shape::Segment;

use crate::math::Point;

/// An approximation of one or more edges
pub struct Approximation {
    pub points: Vec<Point<3>>,
    pub segments: Vec<Segment>,
}
