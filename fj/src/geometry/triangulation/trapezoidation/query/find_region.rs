// TASK: Implement point query (point -> trapezoid containing point):
//       - Start at source, follow path to sink that identifies trapezoid.
//       - At each non-sink, compare with point to decide which path to follow
//         - At X node, whether point is left or right of segment
//         - At Y node, whether point is below or above of segment
