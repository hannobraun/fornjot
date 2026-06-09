use crate::{
    math::Point,
    new::{
        approx::{ApproxHalfEdge, face_approx},
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
        self.segments.push(SketchSegment { to: to.into() });
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
        let _ = surface;

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

        let mut boundary = Vec::new();
        let mut boundary_approx = Vec::new();

        for i in 0..=last_segment_index {
            let prev_i = i.checked_sub(1).unwrap_or(last_segment_index);

            let segment = &self.segments[i];
            let prev = &self.segments[prev_i];

            let vertices = [prev.to, segment.to]
                .map(|point| surface.local_point_to_global(point))
                .map(|point| topology.vertices.push(Vertex { point }));
            let edge = topology.edges.push(Edge {
                boundary: EdgeBoundary { vertices },
                approx: Vec::new(),
            });
            let half_edge = topology.half_edges.push(HalfEdge {
                edge,
                orientation: Orientation::Nominal,
            });
            boundary.push(half_edge);

            let curve = Vec::new();
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
}

#[cfg(test)]
mod tests {
    use crate::{
        math::Point,
        new::{
            geometry::Plane,
            operations::Sketch2,
            topology::{Face, Orientation, Topology},
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

        {
            let [_, _, _] = half_face.boundary.as_slice() else {
                panic!(
                    "Expected half-face to have a boundary of three half-edges."
                );
            };
        }

        {
            let [triangle] = topology.faces[half_face.face].approx.as_slice()
            else {
                panic!(
                    "Expected face approximation to consist of one triangle."
                );
            };

            let [a, b, c] =
                [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]].map(Point::from);

            assert!(
                triangle.points == [a, b, c]
                    || triangle.points == [b, c, a]
                    || triangle.points == [c, a, b]
            );
        }

        assert_eq!(half_face.orientation, Orientation::Nominal);
    }
}
