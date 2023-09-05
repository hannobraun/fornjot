use std::collections::BTreeMap;

use fj_math::Point;

use crate::{geometry::CurveBoundary, objects::Curve, storage::HandleWrapper};

use super::CurveApproxSegment;

/// Cache for curve approximations
#[derive(Default)]
pub struct CurveApproxCache {
    #[allow(missing_docs)]
    pub inner: BTreeMap<
        (HandleWrapper<Curve>, CurveBoundary<Point<1>>),
        CurveApproxSegment,
    >,
}
