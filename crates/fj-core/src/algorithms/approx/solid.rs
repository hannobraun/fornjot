//! Solid approximation

use std::collections::BTreeSet;

use crate::{geometry::Geometry, topology::Solid};

use super::{
    face::FaceApprox, half_edge::HalfEdgeApproxCache, Approx, Tolerance,
};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

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
