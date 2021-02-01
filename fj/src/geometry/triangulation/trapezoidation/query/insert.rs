//! Insertion of segments into the graph

// TASK: Implement insertion:
//       - Unless upper point is already inserted:
//         - Find trapezoid that point is in
//         - Add two new sink nodes
//         - Replace trapezoid node with Y node that points to the new sinks
//       - Unless lower point is already inserted, proceed as with upper point
//       - "Thread" segment through trapezoidation; see paper, unclear right now
