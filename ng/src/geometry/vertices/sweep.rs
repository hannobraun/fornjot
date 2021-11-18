use crate::math::Point;

use super::Vertices;

impl Vertices for fj::Sweep {
    type Vertices = Vec<Point>;

    fn vertices(&self) -> Self::Vertices {
        // TASK Implement.
        todo!()
    }
}
