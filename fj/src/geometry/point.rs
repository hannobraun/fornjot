use std::ops::Deref;

use decorum::R32;
use nalgebra::Point2;

pub struct Pnt2(pub Point2<R32>);

impl Deref for Pnt2 {
    type Target = Point2<R32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
