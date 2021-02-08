// TASK: Implement function that updates all regions after an x node has been
//       inserted:
//       - Replace right segment of new left region.
//       - Replace left segment of new right region.
//       - Remove upper/lower boundary, if bounding point is on wrong side of
//         the new segment. Mark affected regions for merging.
//       - Update boundaries of upper and lower neighbors accordingly.
//       - Merge all regions marked for merging that have the same left/right
//         segment.
// TASK: Implement function that updates all regions after a y node has been
//       inserted:
//       - Replace lower boundary of new upper region.
//       - Replace upper boundary of new lower region.
//       - Update lower neighbors.
