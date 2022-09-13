//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use crate::objects::HalfEdge;

use super::{
    curve::{CurveApprox, RangeOnCurve},
    Approx, ApproxPoint,
};

impl Approx for &HalfEdge {
    type Approximation = HalfEdgeApprox;

    fn approx(self, tolerance: super::Tolerance) -> Self::Approximation {
        let &[a, b] = self.vertices();
        let range = RangeOnCurve::new([a, b]);

        let first = ApproxPoint::new(
            a.surface_form().position(),
            a.global_form().position(),
        );
        let curve_approx = (self.curve(), range).approx(tolerance);

        HalfEdgeApprox {
            first,
            curve_approx,
        }
    }
}

/// An approximation of an [`HalfEdge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct HalfEdgeApprox {
    /// The point that approximates the first vertex of the curve
    pub first: ApproxPoint<2>,

    /// The approximation of the edge's curve
    pub curve_approx: CurveApprox,
}

impl HalfEdgeApprox {
    /// Compute the points that approximate the edge
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        points.push(self.first.clone());
        points.extend(self.curve_approx.points.clone());

        points
    }
}
