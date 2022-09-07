use crate::objects::Edge;

use super::Reverse;

impl Reverse for Edge {
    fn reverse(self) -> Self {
        Edge::from_curve_and_vertices(*self.curve(), self.vertices().reverse())
    }
}
