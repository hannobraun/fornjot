use std::collections::HashSet;

use crate::math::{Point, Scalar, Segment};

use super::topology::{
    edges::{Cycle, Edge, Edges},
    faces::Face,
};

/// An approximation of an edge, multiple edges, or a face
#[derive(Debug, PartialEq)]
pub struct Approximation {
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
    /// field of this struct. This can be verified by calling
    /// [`Approximation::validate`].
    pub segments: HashSet<Segment<3>>,
}

impl Approximation {
    /// Compute an approximate for an edge
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    pub fn for_edge(edge: &Edge, tolerance: Scalar) -> Self {
        let mut points = Vec::new();
        edge.curve.approx(tolerance, &mut points);

        // Insert the exact vertices of this edge into the approximation. This
        // means we don't rely on the curve approximation to deliver accurate
        // representations of these vertices, which they might not be able to
        // do.
        //
        // If we used inaccurate representations of those vertices here, then
        // that would lead to bugs in the approximation, as points that should
        // refer to the same vertex would be understood to refer to very close,
        // but distinct vertices.
        if let Some([a, b]) = edge.vertices {
            points.insert(0, *a.to_canonical().location());
            points.push(*b.to_canonical().location());
        }

        let mut segment_points = points.clone();
        if edge.vertices.is_none() {
            // The edge has no vertices, which means it connects to itself. We
            // need to reflect that in the approximation.

            if let Some(&point) = points.first() {
                segment_points.push(point);
            }
        }

        let mut segments = HashSet::new();
        for segment in segment_points.windows(2) {
            let p0 = segment[0];
            let p1 = segment[1];

            segments.insert(Segment::from([p0, p1]));
        }

        Self {
            points: points.into_iter().collect(),
            segments,
        }
    }

    /// Compute an approximation for a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual cycle.
    pub fn for_cycle(cycle: &Cycle, tolerance: Scalar) -> Self {
        let mut points = HashSet::new();
        let mut segments = HashSet::new();

        for edge in &cycle.edges {
            let approx = Self::for_edge(edge, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        Self { points, segments }
    }

    /// Compute an approximation for multiple edges
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edges.
    pub fn for_edges(edges: &Edges, tolerance: Scalar) -> Self {
        let mut points = HashSet::new();
        let mut segments = HashSet::new();

        for cycle in &edges.cycles {
            let approx = Self::for_cycle(cycle, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        Self { points, segments }
    }

    /// Compute an approximation for a face
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edges.
    #[allow(unused)]
    pub fn for_face(face: &Face, tolerance: Scalar) -> Self {
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
        match face {
            Face::Face { surface: _, edges } => {
                Self::for_edges(edges, tolerance)
            }
            _ => {
                // No code that still uses triangle representation calls this
                // method.
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use map_macro::set;

    use crate::{
        kernel::{
            geometry::{Curve, Surface},
            topology::{
                edges::{Cycle, Edge, Edges},
                faces::Face,
                vertices::Vertex,
            },
        },
        math::{Point, Scalar, Segment},
    };

    use super::Approximation;

    #[test]
    fn for_edge() {
        let tolerance = Scalar::ONE;

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(d);

        let curve = Curve::Mock {
            approx: vec![b, c],
            coords: RefCell::new(vec![Point::from([0.]), Point::from([1.])]),
        };

        let edge_regular = Edge::new(curve.clone(), Some([v1, v2]));
        assert_eq!(
            Approximation::for_edge(&edge_regular, tolerance),
            Approximation {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, d]),
                ],
            }
        );

        let edge_self_connected = Edge::new(curve.clone(), None);
        assert_eq!(
            Approximation::for_edge(&edge_self_connected, tolerance),
            Approximation {
                points: set![b, c],
                segments: set![Segment::from([b, c]), Segment::from([c, b])],
            }
        );
    }

    #[test]
    fn for_cycle() {
        let tolerance = Scalar::ONE;

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);
        let v3 = Vertex::create_at(c);

        let curve = Curve::Mock {
            approx: Vec::new(),
            coords: RefCell::new(vec![Point::from([0.]), Point::from([1.])]),
        };

        let ab = Edge::new(curve.clone(), Some([v1, v2]));
        let bc = Edge::new(curve.clone(), Some([v2, v3]));
        let ca = Edge::new(curve.clone(), Some([v3, v1]));

        let cycle = Cycle {
            edges: vec![ab, bc, ca],
        };

        assert_eq!(
            Approximation::for_cycle(&cycle, tolerance),
            Approximation {
                points: set![a, b, c],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, a]),
                ],
            }
        );
    }

    #[test]
    fn for_edges() {
        let tolerance = Scalar::ONE;

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);
        let v3 = Vertex::create_at(c);
        let v4 = Vertex::create_at(d);

        let curve = Curve::Mock {
            approx: Vec::new(),
            coords: RefCell::new(vec![Point::from([0.]), Point::from([1.])]),
        };

        let ab = Edge::new(curve.clone(), Some([v1, v2]));
        let ba = Edge::new(curve.clone(), Some([v2, v1]));
        let cd = Edge::new(curve.clone(), Some([v3, v4]));
        let dc = Edge::new(curve.clone(), Some([v4, v3]));

        let ab_ba = Cycle {
            edges: vec![ab, ba],
        };
        let cd_dc = Cycle {
            edges: vec![cd, dc],
        };

        let edges = Edges {
            cycles: vec![ab_ba, cd_dc],
        };

        assert_eq!(
            Approximation::for_edges(&edges, tolerance),
            Approximation {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, a]),
                    Segment::from([c, d]),
                    Segment::from([d, c]),
                ],
            }
        );
    }

    #[test]
    fn for_face_closed() {
        // Test a closed face, i.e. one that is completely encircled by edges.

        let tolerance = Scalar::ONE;

        let a = Point::from([1., 2., 3.]);
        let b = Point::from([2., 3., 5.]);
        let c = Point::from([3., 5., 8.]);
        let d = Point::from([5., 8., 13.]);

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);
        let v3 = Vertex::create_at(c);
        let v4 = Vertex::create_at(d);

        let curve = Curve::Mock {
            approx: Vec::new(),
            coords: RefCell::new(vec![Point::from([0.]), Point::from([1.])]),
        };

        let ab = Edge::new(curve.clone(), Some([v1, v2]));
        let bc = Edge::new(curve.clone(), Some([v2, v3]));
        let cd = Edge::new(curve.clone(), Some([v3, v4]));
        let da = Edge::new(curve.clone(), Some([v4, v1]));

        let abcd = Cycle {
            edges: vec![ab, bc, cd, da],
        };

        let edges = Edges { cycles: vec![abcd] };

        let face = Face::Face {
            surface: Surface::x_y_plane(),
            edges,
        };

        assert_eq!(
            Approximation::for_face(&face, tolerance),
            Approximation {
                points: set![a, b, c, d],
                segments: set![
                    Segment::from([a, b]),
                    Segment::from([b, c]),
                    Segment::from([c, d]),
                    Segment::from([d, a]),
                ],
            }
        );
    }
}
