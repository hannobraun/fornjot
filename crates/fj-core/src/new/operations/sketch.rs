use itertools::Itertools;

use crate::{
    math::{Circle, Point, Scalar, Vector},
    new::{
        approx::{ApproxHalfEdge, ApproxPoint, face_approx},
        geometry::Plane,
        topology::{Face, HalfEdge, Handle, Store, Vertex},
    },
};

/// # Draw a 2D sketch and convert it into a face
///
/// You can create a new sketch via [`Sketch::new`], then append segments to it
/// via [`Sketch::arc_to`] and the other methods, then finally convert it into a
/// face via [`Sketch::into_face`], once you're ready.
///
/// ## Origin and destination points
///
/// A sketch is created empty, and the methods that append to it only append
/// segments _to_ a point. The origin point of every segment is implicit, and
/// provided by the destination point of the previous segment. The origin point
/// of the first segment is provided by the destination point of the last one.
///
/// ## Holes
///
/// Sketches consist of a single series of segments that form a sketch's
/// boundary. There is no explicit notion of holes, in the form of additional
/// series of segments that would define those.
///
/// Creating a sketch with holes is still possible though. The holes just need
/// to be part of the single series of segments, which means the sketch's
/// exterior and its holes must be connected.
///
/// To achieve that, you can first create the outside boundary, then, from the
/// point where the last segment of the outside boundary touches its first
/// segment, continue the boundary to the first segment of the hole.
///
/// Between outside boundary and hole, you have a connecting segment, and once
/// you finish the hole, you add a second connecting segment in the other
/// direction, that is coincident with the first one.
///
/// You could use the same principle to create multiple holes, which are
/// connected to the outside boundary in the same location, in different
/// locations, or by connecting the holes to each other. What's important, is
/// that the boundary forms a single, closed cycle in the end.
///
/// ## Shared vertices
///
/// The methods that append sketch segments each come in two variants: a basic
/// one that just lets you define the segment and its definition, and another
/// one that lets you provide the destination [`Vertex`].
///
/// If you do not provide a vertex, then a new one will be created at the
/// segment's destination point. This can lead to distinct but coincident
/// vertices, where you have connecting segments between the exterior boundary
/// and holes, for example, which can be problematic.
///
/// To prevent that, the second set of append methods exists. They allow you to
/// create a single vertex for a set of coincident points in advance, and then
/// provide that vertex for each coincident point, ensuring that all distinct
/// vertices also have distinct positions.
#[derive(Default)]
pub struct Sketch {
    segments: Vec<SketchSegment>,
}

impl Sketch {
    /// # Construct a new instance of `Sketch`
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
        tolerance: impl Into<Scalar>,
        to: impl Into<Point<2>>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: None,
        });

        self
    }

    /// # Add an arc segment to a given point, providing the vertex there
    ///
    /// See [`Sketch::arc_to`] for more information on how the arc gets
    /// constructed. See the documentation of [`Sketch`] for a discussion on why
    /// you might want to provide a vertex.
    pub fn arc_to_at(
        mut self,
        radius: impl Into<Scalar>,
        tolerance: impl Into<Scalar>,
        to: impl Into<Point<2>>,
        at: Handle<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Arc {
                radius: radius.into(),
                tolerance: tolerance.into(),
            },
            attachment: Some(SketchSegmentAttachment::Vertex { vertex: at }),
        });

        self
    }

    /// # Add a line segment to a given point
    pub fn line_to(mut self, to: impl Into<Point<2>>) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Line,
            attachment: None,
        });

        self
    }

    /// # Add a line segment to a given point, providing the vertex there
    ///
    /// See the documentation of [`Sketch`] for a discussion on why you might
    /// want to provide a vertex.
    pub fn line_to_at(
        mut self,
        to: impl Into<Point<2>>,
        at: Handle<Vertex>,
    ) -> Self {
        self.segments.push(SketchSegment {
            end: to.into(),
            geometry: SketchSegmentGeometry::Line,
            attachment: Some(SketchSegmentAttachment::Vertex { vertex: at }),
        });

        self
    }

    /// # Convert the sketch into a face
    ///
    /// A sketch is purely a 2-dimensional construct, with no notion on where
    /// that sketch might be located in a 3D space. In calling this method, the
    /// caller provides provides the surface on which the sketch is to be
    /// located, enabling its conversion into a face.
    pub fn into_face(
        mut self,
        surface: Plane,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
        faces: &mut Store<Face>,
    ) -> Handle<Face> {
        let Some(last_segment_index) = self.segments.len().checked_sub(1)
        else {
            panic!("Empty sketches are not supported yet.");
        };

        let mut boundary = Vec::new();
        let mut boundary_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);
            let next_i = if i == last_segment_index { 0 } else { i + 1 };

            let current = &self.segments[i];
            let prev = &self.segments[prev_i];
            let next = &self.segments[next_i];

            let (half_edge, approx) = current.to_half_edge_and_approx(
                prev, next, &surface, half_edges, vertices,
            );

            boundary.push(half_edge);
            boundary_approx.push(ApproxHalfEdge::from_points(
                prev.end, approx, half_edge, vertices, half_edges,
            ));
            self.segments[i].attachment =
                Some(SketchSegmentAttachment::HalfEdge { half_edge });
        }

        for (&a, &b) in boundary.iter().circular_tuple_windows() {
            assert_eq!(half_edges[a].boundary[1], half_edges[b].boundary[0]);
        }

        let surface_approx = Vec::new();
        let approx = face_approx(&boundary_approx, surface_approx);

        faces.push(Face { boundary, approx })
    }
}

#[derive(Clone, Copy)]
struct SketchSegment {
    pub end: Point<2>,
    pub geometry: SketchSegmentGeometry,
    pub attachment: Option<SketchSegmentAttachment>,
}

impl SketchSegment {
    pub fn to_half_edge_and_approx(
        self,
        prev: &SketchSegment,
        next: &SketchSegment,
        surface: &Plane,
        half_edges: &mut Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> (Handle<HalfEdge>, Vec<ApproxPoint<2>>) {
        let approx = self.geometry.approx(prev.end, self.end, surface);

        let boundary = match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                // We just assume that the approximation of the sketch segment
                // and the existing approximation of the half-edge match. We
                // should make sure by checking it here.
                return (half_edge, approx);
            }
            Some(SketchSegmentAttachment::Vertex { vertex: v1 }) => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);

                [v0, v1]
            }
            None => {
                let v0 = prev.to_end_vertex(surface, half_edges, vertices);
                let v1 = next
                    .to_start_vertex(self.end, surface, half_edges, vertices);

                [v0, v1]
            }
        };

        let half_edge = half_edges.push(HalfEdge {
            boundary,
            approx: approx.iter().copied().map(|point| point.global).collect(),
        });

        (half_edge, approx)
    }

    pub fn to_start_vertex(
        self,
        position: Point<2>,
        surface: &Plane,
        half_edges: &Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Handle<Vertex> {
        match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                let [vertex, _] = half_edges[half_edge].boundary;
                vertex
            }
            Some(SketchSegmentAttachment::Vertex { vertex: _ }) | None => {
                let point = surface.local_point_to_global(position);
                vertices.push(Vertex { point })
            }
        }
    }

    pub fn to_end_vertex(
        self,
        surface: &Plane,
        half_edges: &Store<HalfEdge>,
        vertices: &mut Store<Vertex>,
    ) -> Handle<Vertex> {
        match self.attachment {
            Some(SketchSegmentAttachment::HalfEdge { half_edge }) => {
                let [_, vertex] = half_edges[half_edge].boundary;
                vertex
            }
            Some(SketchSegmentAttachment::Vertex { vertex }) => vertex,
            None => {
                let point = surface.local_point_to_global(self.end);
                vertices.push(Vertex { point })
            }
        }
    }
}

#[derive(Clone, Copy)]
enum SketchSegmentGeometry {
    Arc { radius: Scalar, tolerance: Scalar },
    Line,
}

impl SketchSegmentGeometry {
    fn approx(
        self,
        start: Point<2>,
        end: Point<2>,
        surface: &Plane,
    ) -> Vec<ApproxPoint<2>> {
        let approx = match self {
            SketchSegmentGeometry::Arc { radius, tolerance } => {
                let start_to_end = end - start;

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
                        // TASK: Fix.
                        panic!(
                            "Radius of arc (`{radius}`) is too small: Must be \
                            at least half the distance between start \
                            (`{start:?}`) and end (`{to:?}`) points, or the \
                            arc is not possible.",
                            to = start_to_end,
                        );
                    }

                    b_squared.sqrt()
                };

                let center = start
                    + start_to_end * 0.5
                    + midpoint_towards_center
                        * distance_from_midpoint_to_center;

                // This only works if `surface` is a plane, which checks out for
                // now.
                let circle = {
                    let a = start;
                    let b = center + (a - center).to_perpendicular();

                    Circle::new(center, a - center, b - center)
                };

                let num_vertices_to_approx_full_circle = Scalar::max(
                    Scalar::PI / (Scalar::ONE - (tolerance / radius)).acos(),
                    3.,
                )
                .ceil();

                let increment = Vector::from([
                    Scalar::TAU / num_vertices_to_approx_full_circle
                ]);

                let start_local = circle.point_to_circle_coords(start);
                let end_local =
                    circle.point_to_circle_coords(start + start_to_end);

                let mut approx = Vec::new();

                let mut point = start_local + increment;
                while point < end_local {
                    approx.push(circle.point_from_circle_coords(point));
                    point += increment;
                }

                approx
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

#[derive(Clone, Copy, Debug)]
enum SketchSegmentAttachment {
    HalfEdge { half_edge: Handle<HalfEdge> },
    Vertex { vertex: Handle<Vertex> },
}
