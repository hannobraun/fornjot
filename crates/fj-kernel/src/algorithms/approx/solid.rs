//! Solid approximation

use std::collections::BTreeSet;

use crate::objects::Solid;

use super::{curve::ApproxCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        self.shells()
            .flat_map(|shell| shell.approx_with_cache(tolerance, cache))
            .collect()
    }
}
