use crate::objects::{GlobalEdge, HalfEdge};

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
            self.global_form().clone().reverse(),
        )
    }
}

impl Reverse for GlobalEdge {
    fn reverse(self) -> Self {
        let vertices = {
            let &[a, b] = self.vertices();
            [b, a]
        };

        GlobalEdge::new(self.curve().clone(), vertices)
    }
}
