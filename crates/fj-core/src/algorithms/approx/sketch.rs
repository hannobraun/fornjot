//! Sketch approximation

use std::collections::BTreeSet;

use crate::{geometry::Geometry, topology::Sketch};

use super::{Approx, ApproxCache, Tolerance, face::FaceApprox};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        _tolerance: impl Into<Tolerance>,
        _cache: &mut Self::Cache,
        _: &Geometry,
    ) -> Self::Approximation {
        todo!()
    }
}
