use std::collections::HashSet;

use fj_math::Point;

use crate::objects::Face;

use super::{Approx, CycleApprox, Local, Tolerance};

impl Approx for Face {
    type Approximation = FaceApprox;

    fn approx(&self, tolerance: Tolerance) -> Self::Approximation {
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

        let mut points = HashSet::new();
        let mut exteriors = Vec::new();
        let mut interiors = HashSet::new();

        for cycle in self.exteriors() {
            let cycle = cycle.approx(tolerance);

            points.extend(cycle.points.iter().copied());
            exteriors.push(cycle);
        }
        for cycle in self.interiors() {
            let cycle = cycle.approx(tolerance);

            points.extend(cycle.points.iter().copied());
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
            points,
            exterior,
            interiors,
        }
    }
}

/// An approximation of a [`Face`]
#[derive(Debug, Eq, PartialEq)]
pub struct FaceApprox {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: HashSet<Local<Point<2>>>,

    /// Approximation of the exterior cycle
    pub exterior: CycleApprox,

    /// Approximations of the interior cycles
    pub interiors: HashSet<CycleApprox>,
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar};
    use map_macro::set;

    use crate::{
        algorithms::approx::{Approx, Local},
        objects::{Face, Surface},
    };

    use super::{CycleApprox, FaceApprox, Tolerance};

    #[test]
    fn for_face_closed() -> anyhow::Result<()> {
        // Test a closed face, i.e. one that is completely encircled by edges.

        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;

        let a = Point::from([0., 0.]);
        let b = Point::from([3., 0.]);
        let c = Point::from([3., 3.]);
        let d = Point::from([0., 3.]);

        let e = Point::from([1., 1.]);
        let f = Point::from([2., 1.]);
        let g = Point::from([2., 2.]);
        let h = Point::from([1., 2.]);

        let surface = Surface::xy_plane();
        let face = Face::build(surface)
            .polygon_from_points([a, b, c, d])
            .with_hole([e, f, g, h]);

        let a = Local::new(a, a.to_xyz());
        let b = Local::new(b, b.to_xyz());
        let c = Local::new(c, c.to_xyz());
        let d = Local::new(d, d.to_xyz());
        let e = Local::new(e, e.to_xyz());
        let f = Local::new(f, f.to_xyz());
        let g = Local::new(g, g.to_xyz());
        let h = Local::new(h, h.to_xyz());

        let approx = face.approx(tolerance);
        let expected = FaceApprox {
            points: set![a, b, c, d, e, f, g, h],
            exterior: CycleApprox {
                points: vec![a, b, c, d, a],
            },
            interiors: set![CycleApprox {
                points: vec![e, f, g, h, e],
            }],
        };

        assert_eq!(approx, expected);

        Ok(())
    }
}
