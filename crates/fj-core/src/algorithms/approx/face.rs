//! Face approximation
//!
//! See [`FaceApprox`].

use std::{collections::BTreeSet, ops::Deref};

use crate::{
    approx::Tolerance,
    geometry::Geometry,
    storage::Handle,
    topology::{Face, Handedness, ObjectSet},
    validation::ValidationConfig,
};

use super::{
    Approx, ApproxCache, ApproxPoint,
    cycle::{CycleApprox, approx_cycle},
};

impl Approx for &ObjectSet<Face> {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = ApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        geometry: &Geometry,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        let approx = self
            .into_iter()
            .map(|face| approx_face(face.clone(), tolerance, cache, geometry))
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

/// Approximate the provided face
pub fn approx_face(
    face: Handle<Face>,
    tolerance: impl Into<Tolerance>,
    cache: &mut ApproxCache,
    geometry: &Geometry,
) -> FaceApprox {
    let tolerance = tolerance.into();

    let exterior = approx_cycle(
        face.region().exterior().deref(),
        face.surface(),
        tolerance,
        cache,
        geometry,
    );

    let mut interiors = BTreeSet::new();
    for cycle in face.region().interiors() {
        let cycle = approx_cycle(
            cycle.deref(),
            face.surface(),
            tolerance,
            cache,
            geometry,
        );
        interiors.insert(cycle);
    }

    let coord_handedness = face.coord_handedness(geometry);
    FaceApprox {
        face,
        exterior,
        interiors,
        coord_handedness,
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
