//! Curve approximation

mod approx;
mod points;
mod segment;

pub use self::{
    approx::CurveApprox, points::CurveApproxPoints, segment::CurveApproxSegment,
};
