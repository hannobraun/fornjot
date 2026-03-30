use crate::new::topology::{HalfEdge, HalfFace, Handle, Topology};

/// # Reverse the orientation of a primitive
#[derive(Default)]
pub struct Reverse {}

impl Reverse {
    /// # Construct a new instance of `Reverse`
    pub fn new() -> Self {
        Self::default()
    }

    /// # Reverse the orientation of the provided half-edge
    pub fn half_edge(&mut self, half_edge: &HalfEdge) -> HalfEdge {
        HalfEdge {
            edge: half_edge.edge,
            orientation: half_edge.orientation.reverse(),
        }
    }

    /// # Reverse the orientation of the provided half-face
    pub fn half_face(
        &mut self,
        half_face: Handle<HalfFace>,
        topology: &mut Topology,
    ) -> HalfFace {
        let half_face = &topology.half_faces[half_face];

        let boundary = half_face
            .boundary
            .iter()
            .copied()
            .map(|e| {
                let half_edge = self.half_edge(&topology.half_edges[e]);

                if let Some(index) = half_face
                    .boundary
                    .iter()
                    .copied()
                    .find(|&index| topology.half_edges[index] == half_edge)
                {
                    index
                } else {
                    topology.half_edges.push(half_edge)
                }
            })
            .rev()
            .collect();

        HalfFace {
            boundary,
            face: half_face.face,
            orientation: half_face.orientation.reverse(),
        }
    }
}
