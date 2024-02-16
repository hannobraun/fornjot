//! Sketch approximation

use std::collections::BTreeSet;

use crate::{objects::Sketch, Core};

use super::{edge::HalfEdgeApproxCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        _tolerance: impl Into<Tolerance>,
        _cache: &mut Self::Cache,
        _core: &mut Core,
    ) -> Self::Approximation {
        todo!()
    }
}
