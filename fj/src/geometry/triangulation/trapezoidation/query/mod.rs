//! The point location query structure of the trapezoidation

pub mod graph;

// TASK: Implement point query (point -> trapezoid containing point):
//       - Start at source, follow path to sink that identifies trapezoid.
//       - At each non-sink, compare with point to decide which path to follow
//         - At X node, whether point is left or right of segment
//         - At Y node, whether point is below or above of segment
// TASK: Insertion:
//       - Unless upper point is already inserted:
//         - Find trapezoid that point is in
//         - Add two new sink nodes
//         - Replace trapezoid node with Y node that points to the new sinks
//       - Unless lower point is already inserted, proceed as with upper point
//       - "Thread" segment through trapezoidation; see paper, unclear right now
