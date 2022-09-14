//! Shell approximation

use std::collections::BTreeSet;

use crate::objects::Shell;

use super::{face::FaceApprox, Approx, ApproxCache, Tolerance};

impl Approx for &Shell {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        self.faces().approx_with_cache(tolerance, cache)
    }
}
