use crate::objects::Edge;

use super::Reverse;

impl Reverse for Edge {
    fn reverse(self) -> Self {
        self.reverse_including_curve()
    }
}
