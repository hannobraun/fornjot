use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::Curve,
    storage::{Handle, HandleWrapper},
};

use super::CurveApproxSegment;

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner: BTreeMap<
        (HandleWrapper<Curve>, CurveBoundary<Point<1>>),
        CurveApproxSegment,
    >,
}

impl CurveApproxCache {
    /// Get an approximation from the cache
    pub fn get(
        &self,
        curve: &Handle<Curve>,
        boundary: &CurveBoundary<Point<1>>,
    ) -> Option<CurveApproxSegment> {
        if let Some(approx) =
            self.inner.get(&(curve.clone().into(), *boundary)).cloned()
        {
            return Some(approx);
        }
        if let Some(approx) = self
            .inner
            .get(&(curve.clone().into(), boundary.reverse()))
            .cloned()
        {
            // If we have a cache entry for the reverse boundary, we need to use
            // that too!
            return Some(approx.reverse());
        }

        None
    }

    /// Insert an approximated segment of the curve into the cache
    pub fn insert(
        &mut self,
        curve: Handle<Curve>,
        new_segment: CurveApproxSegment,
    ) -> Option<CurveApproxSegment> {
        self.inner
            .insert((curve.into(), new_segment.boundary), new_segment)
    }
}
