use fj_math::Point;

use crate::geometry::CurveBoundary;

/// A collection of multiple [`CurveBoundary`] instances
///
/// Has a type parameter, `T`, which can be used to attach a payload to each
/// boundary.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveBoundaries<T = ()> {
    /// The [`CurveBoundary`] instances
    pub inner: Vec<(CurveBoundary<Point<1>>, T)>,
}

impl<T> Default for CurveBoundaries<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}
