//! Trapezoidation of polygons
//!
//! Creates sub-polygons that can then be easily split into triangles using an
//! ear-clipping algorithm.

pub mod find_region_for_point;
pub mod find_regions_for_segment;
pub mod graph;
pub mod ids;
pub mod insert;
pub mod point;
pub mod region;
pub mod segment;
pub mod update;

// TASK: Figure out what's missing to complete the implementation, add it.
