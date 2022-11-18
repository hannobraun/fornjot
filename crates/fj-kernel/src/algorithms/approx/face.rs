//! Face approximation
//!
//! See [`FaceApprox`].

use std::collections::BTreeSet;

use fj_interop::mesh::Color;

use crate::{
    objects::{Face, FaceSet, Handedness},
    validate::ValidationConfig,
};

use super::{
    curve::CurveCache, cycle::CycleApprox, Approx, ApproxPoint, Tolerance,
};

impl Approx for &FaceSet {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        let approx = self
            .into_iter()
            .map(|face| face.approx_with_cache(tolerance, cache))
            .collect();

        let min_distance = ValidationConfig::default().distinct_min_distance;
        let mut all_points: BTreeSet<ApproxPoint<2>> = BTreeSet::new();

        // Run some validation code on the approximation.
        for approx in &approx {
            let approx: &FaceApprox = approx;

            for point in &approx.points() {
                for p in &all_points {
                    let distance =
                        (p.global_form - point.global_form).magnitude();

                    if p.global_form != point.global_form
                        && distance < min_distance
                    {
                        let a = p;
                        let b = point;

                        panic!(
                            "Invalid approximation: \
                            Distinct points are too close \
                            (a: {:?}, b: {:?}, distance: {distance})\n\
                            source of `a`: {:#?}\n\
                            source of `b`: {:#?}\n",
                            a.global_form, b.global_form, a.source, b.source
                        );
                    }
                }

                all_points.insert(point.clone());
            }
        }

        approx
    }
}

impl Approx for &Face {
    type Approximation = FaceApprox;
    type Cache = CurveCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        // Curved faces whose curvature is not fully defined by their edges
        // are not supported yet. For that reason, we can fully ignore `face`'s
        // `surface` field and just pass the edges to `Self::for_edges`.
        //
        // An example of a curved face that is supported, is the cylinder. Its
        // curvature is fully defined be the edges (circles) that border it. The
        // circle approximations are sufficient to triangulate the surface.
        //
        // An example of a curved face that is currently not supported, and thus
        // doesn't need to be handled here, is a sphere. A spherical face would
        // would need to provide its own approximation, as the edges that bound
        // it have nothing to do with its curvature.

        let exterior = self.exterior().approx_with_cache(tolerance, cache);

        let mut interiors = BTreeSet::new();
        for cycle in self.interiors() {
            let cycle = cycle.approx_with_cache(tolerance, cache);
            interiors.insert(cycle);
        }

        FaceApprox {
            exterior,
            interiors,
            color: self.color(),
            coord_handedness: self.coord_handedness(),
        }
    }
}

/// An approximation of a [`Face`]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FaceApprox {
    /// Approximation of the exterior cycle
    pub exterior: CycleApprox,

    /// Approximations of the interior cycles
    pub interiors: BTreeSet<CycleApprox>,

    /// The color of the approximated face
    pub color: Color,

    /// The handedness of the approximated face's front-side coordinate system
    pub coord_handedness: Handedness,
}

impl FaceApprox {
    /// Compute all points that make up the approximation
    pub fn points(&self) -> BTreeSet<ApproxPoint<2>> {
        let mut points = BTreeSet::new();

        points.extend(self.exterior.points());

        for cycle_approx in &self.interiors {
            points.extend(cycle_approx.points());
        }

        points
    }
}
