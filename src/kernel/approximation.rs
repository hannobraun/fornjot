use std::{cmp::Ordering, collections::HashSet};

use decorum::R64;
use parry3d_f64::shape::Segment;

use crate::math::Point;

use super::topology::edges::{Cycle, Edge, Edges};

/// An approximation of an edge, multiple edges, or a face
#[derive(Debug, PartialEq)]
pub struct Approximation {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: Vec<Point<3>>,

    /// Segments that approximate edges
    ///
    /// Every approximation will involve edges, typically, and these are
    /// approximated by these segments.
    ///
    /// All the points of these segments will also be available in the `points`
    /// field of this struct. This can be verified by calling
    /// [`Approximation::validate`].
    pub segments: Vec<Segment>,
}

impl Approximation {
    /// Compute an approximate for an edge
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    pub fn for_edge(edge: &Edge, tolerance: f64) -> Self {
        let mut points = Vec::new();
        edge.curve.approx(tolerance, &mut points);

        if edge.reverse {
            points.reverse()
        }

        let mut segment_points = points.clone();
        if edge.vertices.is_none() {
            // The edge has no vertices, which means it connects to itself. We
            // need to reflect that in the approximation.

            if let Some(&point) = points.first() {
                segment_points.push(point);
            }
        }

        let mut segments = Vec::new();
        for segment in segment_points.windows(2) {
            let p0 = segment[0];
            let p1 = segment[1];

            segments.push([p0, p1].into());
        }

        Self { points, segments }
    }

    /// Compute an approximation for a cycle
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual cycle.
    pub fn for_cycle(cycle: &Cycle, tolerance: f64) -> Self {
        let mut points = Vec::new();
        let mut segments = Vec::new();

        for edge in &cycle.edges {
            let approx = Self::for_edge(edge, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        // As this is a cycle, neighboring edges are going to share vertices.
        // Let's remove all those duplicates.
        points.sort_by(|a, b| {
            if a.x < b.x {
                return Ordering::Less;
            }
            if a.x > b.x {
                return Ordering::Greater;
            }
            if a.y < b.y {
                return Ordering::Less;
            }
            if a.y > b.y {
                return Ordering::Greater;
            }
            if a.z < b.z {
                return Ordering::Less;
            }
            if a.z > b.z {
                return Ordering::Greater;
            }

            Ordering::Equal
        });
        points.dedup();

        Self { points, segments }
    }

    /// Compute an approximation for multiple edges
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edges.
    pub fn for_edges(edges: &Edges, tolerance: f64) -> Self {
        let mut points = Vec::new();
        let mut segments = Vec::new();

        for cycle in &edges.cycles {
            let approx = Self::for_cycle(cycle, tolerance);

            points.extend(approx.points);
            segments.extend(approx.segments);
        }

        Self { points, segments }
    }

    /// Validate the approximation
    ///
    /// Returns an `Err(ValidationError)`, if the validation is not valid. See
    /// [`ValidationError`] for the ways that the approximation can be invalid.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut duplicate_points = Vec::new();
        let mut duplicate_segments = Vec::new();
        let mut invalid_segments = Vec::new();
        let mut segments_with_invalid_points = Vec::new();

        // Verify that there are no duplicate points
        let mut points = HashSet::new();
        for &point in &self.points {
            let point_r64 = point_to_r64(point);

            if points.contains(&point_r64) {
                duplicate_points.push(point);
            }

            points.insert(point_r64);
        }

        let mut segments = HashSet::new();
        for &segment @ Segment { a, b } in &self.segments {
            // Verify that there are no duplicate segments
            let ab = [point_to_r64(a), point_to_r64(b)];
            let ba = [point_to_r64(b), point_to_r64(a)];
            if segments.contains(&ab) {
                duplicate_segments.push(segment);
            }
            segments.insert(ab);
            segments.insert(ba);

            // Verify that segments are actually segments
            if a == b {
                invalid_segments.push(segment);
            }

            // Verify that segments refer to valid points
            if !(self.points.contains(&a) && self.points.contains(&b)) {
                segments_with_invalid_points.push(segment);
            }
        }

        if !(duplicate_points.is_empty()
            && duplicate_segments.is_empty()
            && invalid_segments.is_empty()
            && segments_with_invalid_points.is_empty())
        {
            return Err(ValidationError {
                duplicate_points,
                duplicate_segments,
                invalid_segments,
                segments_with_invalid_points,
            });
        }

        Ok(())
    }
}

/// Error returned by [`Approximation::validate`]
#[derive(Debug)]
pub struct ValidationError {
    /// Points that are duplicated
    pub duplicate_points: Vec<Point<3>>,

    /// Segments that are duplicated
    pub duplicate_segments: Vec<Segment>,

    /// Segments that have two equal points
    pub invalid_segments: Vec<Segment>,

    /// Segments that do not refer to points from the approximation
    pub segments_with_invalid_points: Vec<Segment>,
}

fn point_to_r64(point: Point<3>) -> [R64; 3] {
    [point.x.into(), point.y.into(), point.z.into()]
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use nalgebra::point;
    use parry3d_f64::shape::Segment;

    use crate::kernel::{
        geometry::Curve,
        topology::{
            edges::{Cycle, Edge, Edges},
            vertices::Vertex,
        },
    };

    use super::Approximation;

    #[test]
    fn test_for_edge() {
        let tolerance = 1.;

        let a = point![1., 2., 3.];
        let b = point![3., 5., 8.];

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);

        let curve = Curve::Mock {
            approx: vec![a, b],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };

        let edge_regular = Edge::new(curve.clone(), Some([v1, v2]));
        assert_eq!(
            Approximation::for_edge(&edge_regular, tolerance),
            Approximation {
                points: vec![a, b],
                segments: vec![Segment { a, b }],
            }
        );

        let edge_self_connected = Edge::new(curve.clone(), None);
        assert_eq!(
            Approximation::for_edge(&edge_self_connected, tolerance),
            Approximation {
                points: vec![a, b],
                segments: vec![Segment { a, b }, Segment { a: b, b: a }],
            }
        );

        let mut edge_reversed = Edge::new(curve.clone(), Some([v1, v2]));
        edge_reversed.reverse();
        assert_eq!(
            Approximation::for_edge(&edge_reversed, tolerance),
            Approximation {
                points: vec![b, a],
                segments: vec![Segment { a: b, b: a }],
            }
        );
    }

    #[test]
    fn test_for_cycle() {
        let tolerance = 1.;

        let a = point![1., 2., 3.];
        let b = point![2., 3., 5.];
        let c = point![3., 5., 8.];

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);
        let v3 = Vertex::create_at(c);

        let ab = Curve::Mock {
            approx: vec![a, b],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };
        let bc = Curve::Mock {
            approx: vec![b, c],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };
        let ca = Curve::Mock {
            approx: vec![c, a],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };

        let ab = Edge::new(ab, Some([v1, v2]));
        let bc = Edge::new(bc, Some([v2, v3]));
        let ca = Edge::new(ca, Some([v3, v1]));

        let cycle = Cycle {
            edges: vec![ab, bc, ca],
        };

        assert_eq!(
            Approximation::for_cycle(&cycle, tolerance),
            Approximation {
                points: vec![a, b, c],
                segments: vec![
                    Segment { a: a, b: b },
                    Segment { a: b, b: c },
                    Segment { a: c, b: a },
                ],
            }
        );
    }

    #[test]
    fn test_for_edges() {
        let tolerance = 1.;

        let a = point![1., 2., 3.];
        let b = point![2., 3., 5.];
        let c = point![3., 5., 8.];
        let d = point![5., 8., 13.];

        let v1 = Vertex::create_at(a);
        let v2 = Vertex::create_at(b);
        let v3 = Vertex::create_at(c);
        let v4 = Vertex::create_at(d);

        let ab = Curve::Mock {
            approx: vec![a, b],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };
        let ba = Curve::Mock {
            approx: vec![b, a],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };
        let cd = Curve::Mock {
            approx: vec![c, d],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };
        let dc = Curve::Mock {
            approx: vec![d, c],
            coords: RefCell::new(vec![point![0.], point![1.]]),
        };

        let ab = Edge::new(ab, Some([v1, v2]));
        let ba = Edge::new(ba, Some([v2, v1]));
        let cd = Edge::new(cd, Some([v3, v4]));
        let dc = Edge::new(dc, Some([v4, v3]));

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
                points: vec![a, b, c, d],
                segments: vec![
                    Segment { a: a, b: b },
                    Segment { a: b, b: a },
                    Segment { a: c, b: d },
                    Segment { a: d, b: c },
                ],
            }
        );
    }

    #[test]
    fn test_validate() {
        let a = point![0., 1., 2.];
        let b = point![1., 2., 3.];
        let c = point![3., 5., 8.];

        let valid = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }],
        };
        assert!(valid.validate().is_ok());

        let duplicate_points = Approximation {
            points: vec![a, b, c, b],
            segments: vec![Segment { a, b }],
        };
        assert!(duplicate_points.validate().is_err());

        let duplicate_segments = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }, Segment { a, b }],
        };
        assert!(duplicate_segments.validate().is_err());

        let duplicate_segments_inverted = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b }, Segment { a: b, b: a }],
        };
        assert!(duplicate_segments_inverted.validate().is_err());

        let invalid_segment = Approximation {
            points: vec![a, b, c],
            segments: vec![Segment { a, b: a }],
        };
        assert!(invalid_segment.validate().is_err());

        let segment_with_invalid_point = Approximation {
            points: vec![a, c],
            segments: vec![Segment { a, b }],
        };
        assert!(segment_with_invalid_point.validate().is_err());
    }
}
