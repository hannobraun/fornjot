//! Curve approximation
//!
//! Since curves are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.

use std::collections::BTreeMap;

use crate::{
    objects::GlobalCurve,
    storage::{Handle, ObjectId},
};

use super::{path::RangeOnPath, ApproxPoint};

/// An approximation of a [`Curve`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<2>>,
}

impl CurveApprox {
    /// Create an empty instance of `CurveApprox`
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Add points to the approximation
    pub fn with_points(
        mut self,
        points: impl IntoIterator<Item = ApproxPoint<2>>,
    ) -> Self {
        self.points.extend(points);
        self
    }
}

/// A cache for results of an approximation
#[derive(Default)]
pub struct CurveCache {
    inner: BTreeMap<(ObjectId, RangeOnPath), GlobalCurveApprox>,
}

impl CurveCache {
    /// Create an empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert the approximation of a [`GlobalCurve`]
    pub fn insert(
        &mut self,
        handle: Handle<GlobalCurve>,
        range: RangeOnPath,
        approx: GlobalCurveApprox,
    ) -> GlobalCurveApprox {
        self.inner.insert((handle.id(), range), approx.clone());
        approx
    }

    /// Access the approximation for the given [`GlobalCurve`], if available
    pub fn get(
        &self,
        handle: Handle<GlobalCurve>,
        range: RangeOnPath,
    ) -> Option<GlobalCurveApprox> {
        if let Some(approx) = self.inner.get(&(handle.id(), range)) {
            return Some(approx.clone());
        }
        if let Some(approx) = self.inner.get(&(handle.id(), range.reverse())) {
            // If we have a cache entry for the reverse range, we need to use
            // that too!
            return Some(approx.clone().reverse());
        }

        None
    }
}

/// An approximation of a [`GlobalCurve`]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<1>>,
}

impl GlobalCurveApprox {
    /// Reverse the order of the approximation
    pub fn reverse(mut self) -> Self {
        self.points.reverse();
        self
    }
}
