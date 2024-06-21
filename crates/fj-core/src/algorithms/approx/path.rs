//! # Path approximation
//!
//! Since paths are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.
//!
//! ## Determinism
//!
//! Path approximation is carefully designed to produce a deterministic result
//! for the combination of a given path and a given tolerance, regardless of
//! what the range is. This is done to prevent invalid meshes from being
//! generated.
//!
//! In specific terms, this means there is an infinite set of points that
//! approximates a path, and that set is deterministic for a given combination
//! of path and tolerance. The range that defines where the path is approximated
//! only influences the result in two ways:
//!
//! 1. It controls which points from the infinite set are actually computed.
//! 2. It defines the order in which the computed points are returned.
//!
//! As a result, path approximation is guaranteed to generate points that can
//! fit together in a valid mesh, no matter which ranges of a path are being
//! approximated, and how many times.

use fj_math::{Line, Point};

/// Approximate a line
///
/// Since curve approximations don't include the approximation boundary itself,
/// and a line does not require any other points to be fully defined, this
/// method always returns no points.
///
/// The method still exists, to make the code that approximates lines, and thus
/// this piece of documentation, easy to find for anyone who's looking.
pub fn approx_line<const D: usize>(
    line: &Line<D>,
) -> Vec<(Point<1>, Point<D>)> {
    let _ = line;
    Vec::new()
}
