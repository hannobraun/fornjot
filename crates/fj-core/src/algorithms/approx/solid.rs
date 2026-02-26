//! Solid approximation

use std::collections::BTreeSet;

use crate::{approx::Tolerance, geometry::Geometry, topology::Solid};

use super::{Approx, ApproxCache, face::FaceApprox};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        self.shells()
            .iter()
            .flat_map(|shell| {
                shell.approx_with_cache(tolerance, cache, geometry)
            })
            .collect()
    }
}
