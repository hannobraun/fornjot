//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use fj_math::Point;

use crate::objects::Edge;

use super::{curve::RangeOnCurve, Approx};

impl Approx for &Edge {
    type Approximation = Vec<(Point<2>, Point<3>)>;

    fn approx(self, tolerance: super::Tolerance) -> Self::Approximation {
        let boundary = self.vertices().get().map(|&vertex| vertex);
        let range = RangeOnCurve { boundary };

        let mut points = (self.curve(), range).approx(tolerance);
        points.insert(
            0,
            (
                range.start().surface_form().position(),
                range.start().global_form().position(),
            ),
        );

        points
    }
}
