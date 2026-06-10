use crate::{
    approx::{CircleApprox, Tolerance},
    math::{Circle, NonZero, Point, Scalar},
    new::{
        approx::{ApproxHalfEdge, ApproxPoint, face_approx},
        geometry::Plane,
        topology::{
            Edge, EdgeBoundary, Face, HalfEdge, HalfFace, Orientation,
            Topology, Vertex,
        },
    },
};

/// # A new sketch operation
#[derive(Default)]
pub struct Sketch2 {
    segments: Vec<SketchSegment>,
}

impl Sketch2 {
    /// # Construct an empty sketch
    pub fn new() -> Self {
        Self::default()
    }

    /// # Add an arc segment to a given point
    ///
    /// The arc is defined by a given radius, as well as a tolerance value that
    /// defines how far the approximated representation of the arc is allowed to
    /// deviate from the idealized arc.
    ///
    /// If the provided radius is positive, the resulting arc bulges to the
    /// right, when viewed from the origin point of the arc. If the provided
    /// radius is negative, the arc bulges to the left.
    pub fn arc_to(
        mut self,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Tolerance>,
        to: impl Into<Point<2>>,
    ) -> Self {
        self.segments.push(SketchSegment {
            to: to.into(),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
        });

        self
    }

    /// # Add a line segment to a given point
    pub fn line_to(mut self, to: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            to: to.into(),
            geometry: SketchSegmentGeometry::Line,
        });

        self
    }

    /// # Convert the sketch into a half-face
    ///
    /// A sketch is purely a 2-dimensional construct, with no notion of where
    /// that sketch might be located in 3D space. In calling this method, the
    /// caller provides the surface on which the sketch is to be located,
    /// enabling its conversion into a half-face.
    pub fn into_half_face(
        self,
        surface: Plane,
        topology: &mut Topology,
    ) -> HalfFace {
        let Some(last_segment_index) = self.segments.len().checked_sub(1)
        else {
            let empty_boundary = Vec::new();
            let empty_face = topology.faces.push(Face { approx: Vec::new() });

            return HalfFace {
                boundary: empty_boundary,
                face: empty_face,
                orientation: Orientation::Nominal,
            };
        };

        let mut vertices = self
            .segments
            .iter()
            .map(|segment| segment.to)
            .map(|point| surface.local_point_to_global(point))
            .map(|point| topology.vertices.push(Vertex { point }))
            .collect::<Vec<_>>();
        vertices.rotate_right(1);

        let mut boundary = Vec::new();
        let mut boundary_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);

            let prev = &self.segments[prev_i];
            let segment = &self.segments[i];

            let vertices = [vertices[prev_i], vertices[i]];
            let edge = topology.edges.push(Edge {
                boundary: EdgeBoundary { vertices },
                approx: Vec::new(),
            });
            let half_edge = topology.half_edges.push(HalfEdge {
                edge,
                orientation: Orientation::Nominal,
            });
            boundary.push(half_edge);

            let curve = segment.geometry.approx(prev.to, segment.to, &surface);
            boundary_approx.push(ApproxHalfEdge::from_points(
                prev.to, curve, half_edge, topology,
            ));
        }

        let face = {
            // So far, the `surface` parameter is hardcoded to be a `Plane`, so
            // no approximation points are required for that.
            let surface_approx = Vec::new();
            let approx = face_approx(&boundary_approx, surface_approx);
            topology.faces.push(Face { approx })
        };

        HalfFace {
            boundary,
            face,
            orientation: Orientation::Nominal,
        }
    }
}

struct SketchSegment {
    to: Point<2>,
    geometry: SketchSegmentGeometry,
}

enum SketchSegmentGeometry {
    Arc {
        radius: Scalar,
        tolerance: Tolerance,
    },
    Line,
}

impl SketchSegmentGeometry {
    fn approx(
        &self,
        start: Point<2>,
        end: Point<2>,
        surface: &Plane,
    ) -> Vec<ApproxPoint<2>> {
        let _ = start;
        let _ = end;

        let approx = match *self {
            SketchSegmentGeometry::Arc { radius, tolerance } => {
                let Some(start_to_end) = NonZero::new(end - start) else {
                    panic!(
                        "Sketches don't support arcs with identical start and \
                        end points. These don't actually define a full circle, \
                        as one might assume, as only one point and a signed \
                        radius do not provide enough information to locate the \
                        circle center.\n\
                        \n\
                        To create a circle, please construct two subsequent \
                        half-circle arcs instead."
                    );
                };

                let midpoint_towards_center =
                    start_to_end.to_perpendicular().normalize()
                        * radius.sign().to_scalar();

                let distance_from_midpoint_to_center = {
                    // We're computing the required distance from a right
                    // triangle:
                    //
                    // - `a` (leg): `midpoint` to `end`
                    // - `b` (leg): `midpoint` to circle center (the distance
                    //   we're looking for)
                    // - `c` (hypotenuse): `end` to circle center (which is
                    //   `radius`)

                    let a = start_to_end.magnitude() / 2.;
                    let c = radius;

                    let b_squared = c * c - a * a;

                    if b_squared < Scalar::ZERO {
                        panic!(
                            "Radius of arc (`{radius}`) is too small: Must be \
                            at least half the distance between start \
                            (`{start:?}`) and end (`{end:?}`) points, or the \
                            arc is not possible.",
                            end = start_to_end,
                        );
                    }

                    b_squared.sqrt()
                };

                let center = start
                    + start_to_end.into_value() * 0.5
                    + midpoint_towards_center
                        * distance_from_midpoint_to_center;

                // This only works if `surface` is a plane, which checks out for
                // now.
                let circle = {
                    let a = start;
                    let b = center + (a - center).to_perpendicular();

                    Circle::new(center, a - center, b - center)
                };

                let approx = CircleApprox::new(radius, tolerance);

                let start_local = circle.point_to_circle_coords(start);
                let end_local = circle
                    .point_to_circle_coords(start + start_to_end.into_value());

                approx
                    .points([start_local, end_local])
                    .map(|local| circle.point_from_circle_coords(local))
                    .collect()
            }
            SketchSegmentGeometry::Line => Vec::new(),
        };

        approx
            .into_iter()
            .map(|local| {
                let global = surface.local_point_to_global(local);
                ApproxPoint { local, global }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use itertools::Itertools;

    use crate::{
        math::{Point, Scalar},
        new::{
            geometry::Plane,
            operations::Sketch2,
            topology::{Face, HalfFace, Orientation, Topology},
        },
    };

    #[test]
    fn empty() {
        let mut topology = Topology::new();

        let half_face =
            Sketch2::new().into_half_face(Plane::xy(), &mut topology);

        assert_eq!(half_face.boundary, Vec::new());
        assert_eq!(topology.faces[half_face.face], Face { approx: Vec::new() });
        assert_eq!(half_face.orientation, Orientation::Nominal);
    }

    #[test]
    fn triangle() {
        let mut topology = Topology::new();

        let half_face = Sketch2::new()
            .line_to([1., 0.])
            .line_to([0., 1.])
            .line_to([0., 0.])
            .into_half_face(Plane::xy(), &mut topology);

        assert_eq!(half_face.boundary.len(), 3);
        check_connecting_vertices(&half_face, &topology);
        check_approx(
            &half_face,
            &topology,
            1,
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        );
        assert_eq!(half_face.orientation, Orientation::Nominal);
    }

    #[test]
    fn circle() {
        let mut topology = Topology::new();

        let radius = 1.;
        let tolerance = 0.35; // chosen to approximate circle in two triangles

        let half_face = Sketch2::new()
            .arc_to(radius, tolerance, [2., 0.])
            .arc_to(radius, tolerance, [0., 0.])
            .into_half_face(Plane::xy(), &mut topology);

        assert_eq!(half_face.boundary.len(), 2);
        check_connecting_vertices(&half_face, &topology);
        check_approx(
            &half_face,
            &topology,
            2,
            [[0., 0., 0.], [1., -1., 0.], [2., 0., 0.], [1., 1., 0.]],
        );
        assert_eq!(half_face.orientation, Orientation::Nominal);
    }

    fn check_connecting_vertices(half_face: &HalfFace, topology: &Topology) {
        for (prev, half_edge, next) in half_face
            .boundary
            .iter()
            .map(|&half_edge| &topology.half_edges[half_edge])
            .circular_tuple_windows()
        {
            assert_eq!(
                prev.boundary(&topology.edges)[1],
                half_edge.boundary(&topology.edges)[0]
            );
            assert_eq!(
                half_edge.boundary(&topology.edges)[1],
                next.boundary(&topology.edges)[0]
            );

            assert_eq!(half_edge.orientation, Orientation::Nominal);
        }
    }

    fn check_approx(
        half_face: &HalfFace,
        topology: &Topology,
        num_expected_triangles: usize,
        expected_triangle_points: impl IntoIterator<Item = impl Into<Point<3>>>,
    ) {
        let face = &topology.faces[half_face.face];
        assert_eq!(face.approx.len(), num_expected_triangles);

        let mut triangle_points = face
            .approx
            .iter()
            .flat_map(|triangle| triangle.points)
            .collect::<BTreeSet<_>>();

        for expected in expected_triangle_points {
            let expected = expected.into();

            let Some(&point) = triangle_points.iter().find(|&&point| {
                (point - expected).magnitude() < Scalar::from(0.001)
            }) else {
                panic!("Could not find expected point `{expected:?}`.");
            };

            assert!(triangle_points.remove(&point));
        }

        assert!(triangle_points.is_empty());
    }
}
