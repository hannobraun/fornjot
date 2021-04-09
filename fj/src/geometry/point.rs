use std::ops::Deref;

use decorum::R32;
use nalgebra::Point2;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Pnt2(pub Point2<R32>);

impl Pnt2 {
    pub fn from_f32s(x: f32, y: f32) -> Self {
        Point2::new(x, y).into()
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ = (self.0.x, self.0.y);
        let other = (other.0.x, other.0.y);
        self_.cmp(&other)
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
