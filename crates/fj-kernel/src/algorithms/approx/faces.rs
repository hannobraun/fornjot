use std::collections::HashSet;

use crate::{geometry, topology::Face};

use super::{CycleApprox, Tolerance};

/// An approximation of a [`Face`]
#[derive(Debug, PartialEq)]
pub struct FaceApprox {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: HashSet<geometry::Point<3>>,

    /// Approximation of the exterior cycle
    pub exterior: CycleApprox,

    /// Approximations of the interior cycles
    pub interiors: HashSet<CycleApprox>,
}

impl FaceApprox {
    /// Compute the approximation of a face
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual face.
    pub fn new(face: &Face, tolerance: Tolerance) -> Self {
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

        for cycle in face.exteriors() {
            let cycle = CycleApprox::new(&cycle, tolerance);

            points.extend(cycle.points.iter().copied());
            exteriors.push(cycle);
        }
        for cycle in face.interiors() {
            let cycle = CycleApprox::new(&cycle, tolerance);

            points.extend(cycle.points.iter().copied());
            interiors.insert(cycle);
        }

        // Only polygon with exactly one exterior cycle are supported.
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

        Self {
            points,
            exterior,
            interiors,
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar};
    use map_macro::set;

    use crate::{
        geometry::{self, Surface},
        shape::Shape,
        topology::Face,
    };

    use super::{CycleApprox, FaceApprox, Tolerance};

    #[test]
    fn for_face_closed() -> anyhow::Result<()> {
        // Test a closed face, i.e. one that is completely encircled by edges.

        let tolerance = Tolerance::from_scalar(Scalar::ONE).unwrap();

        let mut shape = Shape::new();

        let a = Point::from([0., 0., 0.]);
        let b = Point::from([3., 0., 0.]);
        let c = Point::from([3., 3., 0.]);
        let d = Point::from([0., 3., 0.]);

        let e = Point::from([1., 1., 0.]);
        let f = Point::from([2., 1., 0.]);
        let g = Point::from([2., 2., 0.]);
        let h = Point::from([1., 2., 0.]);

        let face = Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .with_interior_polygon([e, f, g, h])
            .build()?;

        let a = geometry::Point::new(a, a);
        let b = geometry::Point::new(b, b);
        let c = geometry::Point::new(c, c);
        let d = geometry::Point::new(d, d);
        let e = geometry::Point::new(e, e);
        let f = geometry::Point::new(f, f);
        let g = geometry::Point::new(g, g);
        let h = geometry::Point::new(h, h);

        assert_eq!(
            FaceApprox::new(&face.get(), tolerance),
            FaceApprox {
                points: set![a, b, c, d, e, f, g, h],
                exterior: CycleApprox {
                    points: vec![a, b, c, d, a],
                },
                interiors: set![CycleApprox {
                    points: vec![e, f, g, h, e],
                }],
            }
        );

        Ok(())
    }
}
