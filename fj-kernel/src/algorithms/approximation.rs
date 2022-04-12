use std::collections::HashSet;

use fj_math::{Point, Scalar, Segment};

use crate::topology::{Cycle, Face, Vertex};

/// An approximation of a [`Face`]
#[derive(Debug, PartialEq)]
pub struct FaceApprox {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: HashSet<Point<3>>,

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

/// An approximation of a [`Cycle`]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CycleApprox {
    /// The points that approximate the cycle
    pub points: Vec<Point<3>>,
}

impl CycleApprox {
    /// Compute the approximation of a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual face.
    pub fn new(cycle: &Cycle, tolerance: Tolerance) -> Self {
        let mut points = Vec::new();

        for edge in cycle.edges() {
            let mut edge_points = Vec::new();
            edge.curve().approx(tolerance, &mut edge_points);

            points.extend(approximate_edge(edge_points, edge.vertices()));
        }

        points.dedup();

        Self { points }
    }

    /// Construct the segments that approximate the cycle
    pub fn segments(&self) -> Vec<Segment<3>> {
        let mut segments = Vec::new();

        for segment in self.points.windows(2) {
            // This can't panic, as we passed `2` to `windows`. Can be cleaned
            // up, once `array_windows` is stable.
            let p0 = segment[0];
            let p1 = segment[1];

            segments.push(Segment::from([p0, p1]));
        }

        segments
    }
}

fn approximate_edge(
    mut points: Vec<Point<3>>,
    vertices: Option<[Vertex; 2]>,
) -> Vec<Point<3>> {
    // Insert the exact vertices of this edge into the approximation. This means
    // we don't rely on the curve approximation to deliver accurate
    // representations of these vertices, which they might not be able to do.
    //
    // If we used inaccurate representations of those vertices here, then that
    // would lead to bugs in the approximation, as points that should refer to
    // the same vertex would be understood to refer to very close, but distinct
    // vertices.
    if let Some([a, b]) = &vertices {
        points.insert(0, a.point());
        points.push(b.point());
    }

    if vertices.is_none() {
        // The edge has no vertices, which means it connects to itself. We need
        // to reflect that in the approximation.

        if let Some(&point) = points.first() {
            points.push(point);
        }
    }

    points
}

impl<T, P> From<T> for CycleApprox
where
    T: IntoIterator<Item = P>,
    P: Into<Point<3>>,
{
    fn from(points: T) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}

/// A tolerance value
///
/// A tolerance value is used during approximation. It defines the maximum
/// allowed deviation of the approximation from the actual shape.
///
/// The `Tolerance` type enforces that the tolerance value is always larger than
/// zero, which is an attribute that the approximation code relies on.
///
/// # Failing [`From`]/[`Into`] implementation
///
/// The [`From`]/[`Into`] implementations of tolerance are fallible, which goes
/// against the explicit mandate of those traits, as stated in their
/// documentation.
///
/// A fallible [`Into`] provides a lot of convenience in test code. Since said
/// documentation doesn't provide any actual reasoning for this requirement, I'm
/// feeling free to just ignore it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Tolerance(Scalar);

impl Tolerance {
    /// Construct a `Tolerance` from a [`Scalar`]
    ///
    /// Returns an error, if the passed scalar is not larger than zero.
    pub fn from_scalar(scalar: impl Into<Scalar>) -> Result<Self, Scalar> {
        let scalar = scalar.into();

        if scalar <= Scalar::ZERO {
            return Err(scalar);
        }

        Ok(Self(scalar))
    }

    /// Return the [`Scalar`] that defines the tolerance
    pub fn inner(&self) -> Scalar {
        self.0
    }
}

impl<S> From<S> for Tolerance
where
    S: Into<Scalar>,
{
    fn from(scalar: S) -> Self {
        Self::from_scalar(scalar).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar};
    use map_macro::set;

    use crate::{
        geometry::Surface,
        shape::Shape,
        topology::{Face, Vertex},
    };

    use super::{CycleApprox, FaceApprox, Tolerance};

    #[test]
    fn approximate_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = Vertex::builder(&mut shape).build_from_point(a)?;
        let v2 = Vertex::builder(&mut shape).build_from_point(d)?;

        // Regular edge
        assert_eq!(
            super::approximate_edge(vec![b, c], Some([v1.get(), v2.get()])),
            vec![a, b, c, d],
        );

        // Continuous edge
        assert_eq!(super::approximate_edge(vec![b, c], None), vec![b, c, b],);

        Ok(())
    }

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

        let face = Face::builder(Surface::x_y_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .with_interior_polygon([e, f, g, h])
            .build()?;

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
