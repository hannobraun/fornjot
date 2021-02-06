//! Insertion of segments into the graph

pub mod insert_point;

// TASK: Implement `insert`
//       - Insert upper point
//       - Insert lower point
//       - Insert segment
// TASK: Decide where to update region when inserting.
// TASK: After insertion, merge regions that have the same bounding segments.
// TASK: Implement `insert_segment`:
//       - Compare with each node
//         - At Y node:
//           - If above or below, follow respective path
//           - If neither, follow _both_ paths
//         - At X node: Check whether left or right, follow respective path
//           - If segments overlap in y, "left" and "right" are unambiguously
//             defined depending on whether the left or right horizontal
//             extension of the overlapping endpoint hit the other segment. Just
//             find an endpoint that is between the other segment's endpoints,
//             find point on other segment with same y-coordinate,
//             compare x-coordinate.
//           - Endpoints of a segment can never be _on_ another segment, unless
//             the initial polygon is self-intersecting, which we don't accept
//             as valid input. (And the paper only allows segments with common
//             endpoints, at most.)
//           - But what if the segments don't overlap in y? There obviously are
//             segments like that, but will we ever have to compare them when
//             inserting into a valid tree? I don't know.
//         - At Sink:
//           Split region into left and right. Update bounding segments.
