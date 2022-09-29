use crate::objects::HalfEdge;

use super::Reverse;

impl Reverse for HalfEdge {
    fn reverse(self) -> Self {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        HalfEdge::new(
            self.curve().clone(),
            vertices,
            self.global_form().clone(),
        )
    }
}
