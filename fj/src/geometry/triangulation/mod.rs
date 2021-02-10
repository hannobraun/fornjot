//! Triangulation of polygons based on Seidel's 1991 paper
//!
//! Based on "A simple and fast incremental randomized algorithm for computing
//! trapezoidal decompositions and for triangulating polygons", by Raimund
//! Seidel.

pub mod diagonalization;
pub mod ear_clipping;
pub mod trapezoidation;

// TASK: Add top-level function here that puts everything together?
