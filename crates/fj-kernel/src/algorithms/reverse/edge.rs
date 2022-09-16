use crate::objects::HalfEdge;

use super::Reverse;

impl Reverse for HalfEdge {
    fn reverse(self) -> Self {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        HalfEdge::from_curve_and_vertices(self.curve().clone(), vertices)
    }
}
