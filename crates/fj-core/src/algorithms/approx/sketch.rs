//! Sketch approximation

use std::collections::BTreeSet;

use crate::{geometry::Geometry, topology::Sketch};

use super::{
    face::FaceApprox, half_edge::HalfEdgeApproxCache, Approx, Tolerance,
};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        _tolerance: impl Into<Tolerance>,
        _cache: &mut Self::Cache,
        _: &Geometry,
    ) -> Self::Approximation {
        todo!()
    }
}
