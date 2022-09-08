//! Face approximation
//!
//! See [`FaceApprox`].

use std::collections::BTreeSet;

use fj_interop::mesh::Color;

use crate::{
    algorithms::validate::ValidationConfig,
    objects::{Face, Faces},
};

use super::{cycle::CycleApprox, Approx, ApproxPoint, Tolerance};

impl Approx for &Faces {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        let approx = self
            .into_iter()
            .map(|face| face.approx(tolerance))
            .collect();

        let min_distance = ValidationConfig::default().distinct_min_distance;
        let mut all_points: BTreeSet<ApproxPoint<2>> = BTreeSet::new();

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

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
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

        let mut exteriors = Vec::new();
        let mut interiors = BTreeSet::new();

        for cycle in self.exteriors() {
            let cycle = cycle.approx(tolerance);
            exteriors.push(cycle);
        }
        for cycle in self.interiors() {
            let cycle = cycle.approx(tolerance);
            interiors.insert(cycle);
        }

        // Only polygons with exactly one exterior cycle are supported.
        //
        // See this issue for some background:
        // https://github.com/hannobraun/Fornjot/issues/250
        let exterior = exteriors
            .pop()
            .expect("Can't approximate face without exterior cycle");
        assert!(
            exteriors.is_empty(),
            "Approximation only supports faces with one exterior cycle",
        );

        FaceApprox {
            exterior,
            interiors,
            color: self.color(),
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
