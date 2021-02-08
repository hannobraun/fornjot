use super::{graph::Graph, ids::Id};

pub fn update_after_x_split(_x: Id, _graph: &mut Graph) {
    // TASK: Implement:
    //       - Replace right segment of new left region.
    //       - Replace left segment of new right region.
    //       - Remove upper/lower boundary, if bounding point is on wrong side
    //         of the new segment. Mark affected regions for merging.
    //       - Update boundaries of upper and lower neighbors accordingly.
    //       - Merge all regions marked for merging that have the same left/
    //         right segment.
}

pub fn update_after_y_split(_y: Id, _graph: &mut Graph) {
    // TASK: Implement:
    //       - Replace lower boundary of new upper region.
    //       - Replace upper boundary of new lower region.
    //       - Update lower neighbors.
    todo!()
}
