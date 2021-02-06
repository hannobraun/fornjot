use crate::geometry::triangulation::trapezoidation::{
    query::graph::{Graph, Id, X, Y},
    segment::Segment,
};

/// Find the regions that are split by the given segment
pub fn find_regions_for_segment<Region>(
    _segment: &Segment,
    _graph: &Graph<X, Y, Region>,
) -> Vec<Id> {
    // TASK: Implement
    todo!()
}
