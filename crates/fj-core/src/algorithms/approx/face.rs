//! Face approximation
//!
//! See [`FaceApprox`].

use std::{collections::BTreeSet, ops::Deref};

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Face, Handedness, ObjectSet},
    validation::ValidationConfig,
};

use super::{
    cycle::CycleApprox, half_edge::HalfEdgeApproxCache, Approx, ApproxPoint,
    Tolerance,
};

impl Approx for &ObjectSet<Face> {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        let approx = self
            .into_iter()
            .map(|face| {
                face.clone().approx_with_cache(tolerance, cache, geometry)
            })
            .collect();

        let min_distance = ValidationConfig::default().distinct_min_distance;
        let mut all_points: BTreeSet<ApproxPoint<2>> = BTreeSet::new();

        // Run some validation code on the approximation.
        for approx in &approx {
            let approx: &FaceApprox = approx;

            for a in &approx.points() {
                for b in &all_points {
                    let distance = (b.global_form - a.global_form).magnitude();

                    if b.global_form != a.global_form && distance < min_distance
                    {
                        panic!(
                            "Invalid approximation: \
                            Distinct points are too close \
                            (a: {:?}, b: {:?}, distance: {distance})",
                            a.global_form, b.global_form,
                        );
                    }
                }

                all_points.insert(*a);
            }
        }

        approx
    }
}

impl Approx for Handle<Face> {
    type Approximation = FaceApprox;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
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

        let exterior = (self.region().exterior().deref(), self.surface())
            .approx_with_cache(tolerance, cache, geometry);

        let mut interiors = BTreeSet::new();
        for cycle in self.region().interiors() {
            let cycle = (cycle.deref(), self.surface())
                .approx_with_cache(tolerance, cache, geometry);
            interiors.insert(cycle);
        }

        let coord_handedness = self.coord_handedness(geometry);
        FaceApprox {
            face: self,
            exterior,
            interiors,
            coord_handedness,
        }
    }
}

/// An approximation of a [`Face`]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FaceApprox {
    /// The [`Face`], that this approximates
    pub face: Handle<Face>,

    /// Approximation of the exterior cycle
    pub exterior: CycleApprox,

    /// Approximations of the interior cycles
    pub interiors: BTreeSet<CycleApprox>,

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
