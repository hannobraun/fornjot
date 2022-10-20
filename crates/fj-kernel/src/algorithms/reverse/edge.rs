use crate::objects::{HalfEdge, Objects};

use super::Reverse;

impl Reverse for HalfEdge {
    fn reverse(self, _: &Objects) -> Self {
        let vertices = {
            let [a, b] = self.vertices().clone();
            [b, a]
        };

        HalfEdge::new(vertices, self.global_form().clone())
    }
}
