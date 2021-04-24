use std::{cmp::Ordering, fmt, ops::Deref};

use decorum::R32;
use nalgebra::Point;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pnt<const D: usize>(pub Point<R32, D>);

impl Pnt<2> {
    pub fn new(x: impl Into<R32>, y: impl Into<R32>) -> Self {
        Self(Point::<_, 2>::new(x.into(), y.into()))
    }
}

impl<const D: usize> Deref for Pnt<D> {
    type Target = Point<R32, D>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TASK: `Point2` doesn't implement `Ord`, even if its type parameter does. This
//       should be fixed in nalgebra.
// TASK: Make generic over dimension.
impl Ord for Pnt<2> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_ = (self.0.x, self.0.y);
        let other = (other.0.x, other.0.y);
        self_.cmp(&other)
    }
}

// TASK: Make generic over dimension.
impl PartialOrd for Pnt<2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_ = (self.0.x, self.0.y);
        let other = (other.0.x, other.0.y);
        self_.partial_cmp(&other)
    }
}

// TASK: Make generic over dimension.
impl fmt::Debug for Pnt<2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

// TASK: Make generic over dimension.
impl fmt::Display for Pnt<2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl<const D: usize> From<&Pnt<D>> for Pnt<D> {
    fn from(point: &Pnt<D>) -> Self {
        *point
    }
}

impl<const D: usize> From<Point<f32, D>> for Pnt<D> {
    fn from(point: Point<f32, D>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl<const D: usize> From<&Point<f32, D>> for Pnt<D> {
    fn from(point: &Point<f32, D>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl<const D: usize> From<Pnt<D>> for Point<f32, D> {
    fn from(point: Pnt<D>) -> Self {
        point.map(|value| value.into_inner())
    }
}

impl<const D: usize> From<&Pnt<D>> for Point<f32, D> {
    fn from(point: &Pnt<D>) -> Self {
        point.map(|value| value.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::Pnt;

    #[test]
    fn points_should_have_defined_order() {
        let a = Pnt::new(0.0, 1.0);
        let b = Pnt::new(1.0, 0.0);

        assert_eq!(a > b, false);
        assert_eq!(a < b, true);
        assert_eq!(b > a, true);
        assert_eq!(b < a, false);
    }
}
