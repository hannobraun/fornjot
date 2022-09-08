use crate::objects::Edge;

use super::Reverse;

impl Reverse for Edge {
    fn reverse(self) -> Self {
        let vertices = {
            let &[a, b] = self.vertices();
            [b, a]
        };

        Edge::from_curve_and_vertices(*self.curve(), vertices)
    }
}
