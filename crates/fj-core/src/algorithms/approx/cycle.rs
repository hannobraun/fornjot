//! Cycle approximation
//!
//! See [`CycleApprox`].

use fj_math::Segment;

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Surface},
};

use super::{
    half_edge::{HalfEdgeApprox, HalfEdgeApproxCache},
    Approx, ApproxPoint, Tolerance,
};

impl Approx for (&Cycle, &Handle<Surface>) {
    type Approximation = CycleApprox;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let (cycle, surface) = self;
        let tolerance = tolerance.into();

        let half_edges = cycle
            .half_edges()
            .iter()
            .map(|half_edge| {
                (half_edge, surface)
                    .approx_with_cache(tolerance, cache, geometry)
            })
            .collect();

        CycleApprox { half_edges }
    }
}

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CycleApprox {
    /// The approximated half-edges that make up the approximated cycle
    pub half_edges: Vec<HalfEdgeApprox>,
}

impl CycleApprox {
    /// Compute the points that approximate the cycle
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        for approx in &self.half_edges {
            points.extend(approx.points.iter().copied());
        }

        if let Some(point) = points.first() {
            points.push(*point);
        }

        points
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points().windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let segment = [&segment[0], &segment[1]];

            segments
                .push(Segment::from(segment.map(|point| point.global_form)));
        }

        segments
    }
}
