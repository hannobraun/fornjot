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
        let was_already_normalized = boundary.is_normalized();
        let normalized_boundary = boundary.normalize();

        self.inner
            .get(&(curve.clone().into(), normalized_boundary))
            .cloned()
            .map(|approx| {
                if was_already_normalized {
                    approx
                } else {
                    approx.reverse()
                }
            })
    }

    /// Insert an approximated segment of the curve into the cache
    pub fn insert(
        &mut self,
        curve: Handle<Curve>,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let new_segment = new_segment.normalize();
        self.inner
            .insert((curve.into(), new_segment.boundary), new_segment.clone())
            .unwrap_or(new_segment)
    }
}
