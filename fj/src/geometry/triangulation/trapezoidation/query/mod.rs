//! The point location query structure of the trapezoidation

pub mod find_region_for_point;
pub mod find_regions_for_segment;
pub mod graph;
pub mod insert_point;
pub mod insert_segment;

// TASK: Implement `insert`
//       - Insert upper point
//       - Insert lower point
//       - Insert segment
// TASK: Decide where to update region when inserting.
// TASK: After insertion, merge regions that have the same bounding segments.
// TASK: Consider merging this module into its parent.
