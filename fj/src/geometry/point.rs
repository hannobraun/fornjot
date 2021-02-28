use std::ops::Deref;

use decorum::R32;
use nalgebra::Point2;

#[derive(Clone, Copy)]
pub struct Pnt2(pub Point2<R32>);

impl Deref for Pnt2 {
    type Target = Point2<R32>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
