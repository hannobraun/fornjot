//! Shell approximation

use std::collections::BTreeSet;

use crate::{topology::Shell, Core};

use super::{edge::HalfEdgeApproxCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Shell {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        core: &mut Core,
    ) -> Self::Approximation {
        self.faces().approx_with_cache(tolerance, cache, core)
    }
}
