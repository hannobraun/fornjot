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

    /// Segments that approximate edges
    ///
    /// Every approximation will involve edges, typically, and these are
    /// approximated by these segments.
    ///
    /// All the points of these segments will also be available in the `points`
    /// field of this struct.
    pub segments: HashSet<Segment<3>>,
}

impl FaceApprox {
    /// Compute the approximation of a face
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual face.
    pub fn new(face: &Face, tolerance: Scalar) -> Self {
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
        let mut segments = HashSet::new();

        for cycle in face.all_cycles() {
            let cycle = CycleApprox::new(&cycle, tolerance);

            let mut cycle_segments = Vec::new();
            for segment in cycle.points.windows(2) {
                let p0 = segment[0];
                let p1 = segment[1];

                cycle_segments.push(Segment::from([p0, p1]));
            }

            points.extend(cycle.points);
            segments.extend(cycle_segments);
        }

        Self { points, segments }
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
    pub fn new(cycle: &Cycle, tolerance: Scalar) -> Self {
        let mut points = Vec::new();

        for edge in cycle.edges() {
            let mut edge_points = Vec::new();
            edge.curve().approx(tolerance, &mut edge_points);

            points.extend(approximate_edge(edge_points, edge.vertices()));
        }

        points.dedup();

        Self { points }
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

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar, Segment};
    use map_macro::set;

    use crate::{
        geometry::Surface,
        shape::Shape,
        topology::{Face, Vertex},
    };

    use super::FaceApprox;

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

        let tolerance = Scalar::ONE;

        let mut shape = Shape::new();

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let face = Face::builder(Surface::x_y_plane(), &mut shape)
            .with_exterior_polygon([a, b, c, d])
            .build()?;

        assert_eq!(
            FaceApprox::new(&face.get(), tolerance),
            FaceApprox {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, d]),
                    Segment::from([d, a]),
                ],
            }
        );

        Ok(())
    }
}
