use fj_math::Triangle;

use crate::new::topology::{Face, HalfEdge, Store};

#[derive(Default)]
pub struct Reverse {}

impl Reverse {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn half_edge(&mut self, half_edge: &HalfEdge) -> HalfEdge {
        let HalfEdge {
            mut boundary,
            mut approx,
        } = half_edge.clone();

        boundary.reverse();
        approx.reverse();

        HalfEdge { boundary, approx }
    }

    pub fn face(
        &mut self,
        face: &Face,
        half_edges: &mut Store<HalfEdge>,
    ) -> Face {
        let boundary = face
            .boundary
            .iter()
            .copied()
            .map(|e| {
                let half_edge = self.half_edge(&half_edges[e]);

                if let Some(index) = face
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

        let approx = face
            .approx
            .iter()
            .copied()
            .map(Triangle::reverse)
            .rev()
            .collect();

        Face { boundary, approx }
    }
}
