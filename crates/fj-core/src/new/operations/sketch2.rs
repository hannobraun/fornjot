use crate::{
    math::Point,
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

        let approx = match self {
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
        math::Point,
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

        for point in expected_triangle_points {
            let point = point.into();
            assert!(triangle_points.remove(&point));
        }

        assert!(triangle_points.is_empty());
    }
}
