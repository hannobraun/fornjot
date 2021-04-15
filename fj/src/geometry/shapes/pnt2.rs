use std::{cmp::Ordering, fmt, ops::Deref};

use decorum::R32;
use nalgebra::Point2;

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct Pnt2(pub Point2<R32>);

impl Pnt2 {
    pub fn new(x: impl Into<R32>, y: impl Into<R32>) -> Self {
        Self(Point2::new(x.into(), y.into()))
    }
}

impl Deref for Pnt2 {
    type Target = Point2<R32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TASK: `Point2` doesn't implement `Ord`, even if its type parameter does. This
//       should be fixed in nalgebra.
impl Ord for Pnt2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_ = (self.0.x, self.0.y);
        let other = (other.0.x, other.0.y);
        self_.cmp(&other)
    }
}

impl fmt::Debug for Pnt2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl From<&Pnt2> for Pnt2 {
    fn from(point: &Pnt2) -> Self {
        *point
    }
}

impl From<Point2<f32>> for Pnt2 {
    fn from(point: Point2<f32>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl From<&Point2<f32>> for Pnt2 {
    fn from(point: &Point2<f32>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl From<Pnt2> for Point2<f32> {
    fn from(point: Pnt2) -> Self {
        point.map(|value| value.into_inner())
    }
}

impl From<&Pnt2> for Point2<f32> {
    fn from(point: &Pnt2) -> Self {
        point.map(|value| value.into_inner())
    }
}
