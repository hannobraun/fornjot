//! Sketch approximation

use std::collections::BTreeSet;

use crate::objects::Sketch;

use super::{edge::EdgeCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = EdgeCache;

    fn approx_with_cache(
        self,
        _tolerance: impl Into<Tolerance>,
        _cache: &mut Self::Cache,
    ) -> Self::Approximation {
        todo!()
    }
}
