use crate::{
    math::Triangle,
    new::topology::{HalfEdge, HalfFace, Store},
};

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
        half_face: &HalfFace,
        half_edges: &mut Store<HalfEdge>,
    ) -> HalfFace {
        let boundary = half_face
            .boundary
            .iter()
            .copied()
            .map(|e| {
                let half_edge = self.half_edge(&half_edges[e]);

                if let Some(index) = half_face
                    .boundary
                    .iter()
                    .copied()
                    .find(|&index| half_edges[index] == half_edge)
                {
                    index
                } else {
                    half_edges.push(half_edge)
                }
            })
            .rev()
            .collect();

        let approx = half_face
            .approx
            .iter()
            .copied()
            .map(Triangle::reverse)
            .rev()
            .collect();

        HalfFace { boundary, approx }
    }
}
