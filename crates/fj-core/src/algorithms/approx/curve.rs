//! Curve approximation

use std::collections::BTreeMap;

use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::Curve,
    storage::{Handle, HandleWrapper},
};

use super::ApproxPoint;

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    inner: BTreeMap<
        (HandleWrapper<Curve>, CurveBoundary<Point<1>>),
        Vec<ApproxPoint<1>>,
    >,
}

impl CurveApproxCache {
    /// Get an approximated curve from the cache
    pub fn get(
        &self,
        handle: &Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
    ) -> Option<Vec<ApproxPoint<1>>> {
        let handle = HandleWrapper::from(handle.clone());

        if let Some(approx) = self.inner.get(&(handle.clone(), boundary)) {
            return Some(approx.clone());
        }
        if let Some(approx) = self.inner.get(&(handle, boundary.reverse())) {
            let mut approx = approx.clone();
            approx.reverse();

            return Some(approx);
        }

        None
    }

    /// Insert an approximated curve into the cache
    pub fn insert(
        &mut self,
        handle: Handle<Curve>,
        boundary: CurveBoundary<Point<1>>,
        approx: Vec<ApproxPoint<1>>,
    ) -> Vec<ApproxPoint<1>> {
        let handle = HandleWrapper::from(handle);
        self.inner
            .insert((handle, boundary), approx.clone())
            .unwrap_or(approx)
    }
}
