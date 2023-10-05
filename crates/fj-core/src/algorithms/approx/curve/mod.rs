//! Curve approximation

mod approx;
mod cache;
mod points;
mod segment;

pub use self::{
    approx::CurveApprox, cache::CurveApproxCache, points::CurveApproxPoints,
    segment::CurveApproxSegment,
};
