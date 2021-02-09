use crate::geometry::triangulation::trapezoidation::graph::{Graph, X};

pub fn update(_x: X, _graph: &mut Graph) {
    // TASK: Implement:
    //       - Replace right segment of new left region.
    //       - Replace left segment of new right region.
    //       - Remove upper/lower boundary, if bounding point is on wrong side
    //         of the new segment. Mark affected regions for merging.
    //       - Update boundaries of upper and lower neighbors accordingly.
    //       - Merge all regions marked for merging that have the same left/
    //         right segment.
    todo!()
}
