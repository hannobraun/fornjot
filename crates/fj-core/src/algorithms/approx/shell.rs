//! Shell approximation

use std::collections::BTreeSet;

use crate::{geometry::Geometry, topology::Shell};

use super::{face::FaceApprox, Approx, ApproxCache, Tolerance};

impl Approx for &Shell {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        self.faces().approx_with_cache(tolerance, cache, geometry)
    }
}
