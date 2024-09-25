use fj_math::Point;

use crate::geometry::curves::line::Line;

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
