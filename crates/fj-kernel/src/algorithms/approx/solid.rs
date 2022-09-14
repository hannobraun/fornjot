//! Solid approximation

use std::collections::BTreeSet;

use crate::objects::Solid;

use super::{face::FaceApprox, Approx, ApproxCache, Tolerance};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx_with_cache(
        self,
        tolerance: Tolerance,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        self.shells()
            .flat_map(|shell| shell.approx_with_cache(tolerance, cache))
            .collect()
    }
}
