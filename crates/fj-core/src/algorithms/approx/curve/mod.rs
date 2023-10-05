//! Curve approximation

mod approx;
mod cache;
mod segment;

pub use self::{
    approx::CurveApprox, cache::CurveApproxCache, segment::CurveApproxSegment,
};
