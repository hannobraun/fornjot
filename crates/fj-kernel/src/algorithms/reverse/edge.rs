use crate::objects::{Edge, VerticesOfEdge};

use super::Reverse;

impl Reverse for Edge {
    fn reverse(self) -> Self {
        let vertices = {
            let &VerticesOfEdge([a, b]) = self.vertices();
            VerticesOfEdge([b, a])
        };

        Edge::from_curve_and_vertices(*self.curve(), vertices)
    }
}
