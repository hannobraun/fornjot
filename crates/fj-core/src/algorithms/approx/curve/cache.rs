use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::Curve,
    storage::{Handle, HandleWrapper},
};

use super::{CurveApprox, CurveApproxSegment};

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner:
        BTreeMap<(HandleWrapper<Curve>, CurveBoundary<Point<1>>), CurveApprox>,
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
            .map(|mut approx| {
                if was_already_normalized {
                    approx
                } else {
                    for segment in &mut approx.segments {
                        *segment = segment.clone().reverse();
                    }
                    approx
                }
            })
            .map(|approx| {
                let mut segments = approx.segments.into_iter();
                let segment = segments
                    .next()
                    .expect("Empty approximations should not exist in cache");
                assert!(
                    segments.next().is_none(),
                    "Cached approximations should have exactly 1 segment"
                );
                segment
            })
    }

    /// Insert an approximated segment of the curve into the cache
    pub fn insert(
        &mut self,
        curve: Handle<Curve>,
        new_segment: CurveApproxSegment,
    ) -> CurveApproxSegment {
        let new_segment = new_segment.normalize();

        // We assume that curve approximation segments never overlap, so so we
        // don't have to do any merging of this segment with a possibly existing
        // approximation for this curve.
        //
        // For now, this is a valid assumption, as it matches the uses of this
        // method, due to documented limitations elsewhere in the system.
        let approx = CurveApprox {
            segments: vec![new_segment.clone()],
        };

        self.inner
            .insert((curve.into(), new_segment.boundary), approx)
            .map(|approx| {
                let mut segments = approx.segments.into_iter();
                let segment = segments
                    .next()
                    .expect("Empty approximations should not exist in cache");
                assert!(
                    segments.next().is_none(),
                    "Cached approximations should have exactly 1 segment"
                );
                segment
            })
            .unwrap_or(new_segment)
    }
}
