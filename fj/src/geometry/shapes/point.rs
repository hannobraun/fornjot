use std::{cmp::Ordering, fmt, ops::Deref};

use decorum::R32;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point<const D: usize>(pub nalgebra::Point<R32, D>);

impl Point<2> {
    pub fn new(x: impl Into<R32>, y: impl Into<R32>) -> Self {
        Self(nalgebra::Point::<_, 2>::new(x.into(), y.into()))
    }
}

impl<const D: usize> Deref for Point<D> {
    type Target = nalgebra::Point<R32, D>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TASK: `Point` doesn't implement `Ord`, even if its type parameter does. This
//       should be fixed in nalgebra.
impl<const D: usize> Ord for Point<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_: [R32; D] = self.into();
        let other: [R32; D] = other.into();
        self_.cmp(&other)
    }
}

impl<const D: usize> PartialOrd for Point<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_: [R32; D] = self.into();
        let other: [R32; D] = other.into();
        self_.partial_cmp(&other)
    }
}

impl<const D: usize> fmt::Debug for Point<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl<const D: usize> fmt::Display for Point<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let self_: [R32; D] = self.into();

        write!(f, "(")?;
        for (i, v) in self_.iter().enumerate() {
            write!(f, "{}", v)?;
            if i < self_.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl<const D: usize> From<&Point<D>> for Point<D> {
    fn from(point: &Point<D>) -> Self {
        *point
    }
}

impl<const D: usize> From<nalgebra::Point<f32, D>> for Point<D> {
    fn from(point: nalgebra::Point<f32, D>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl<const D: usize> From<&nalgebra::Point<f32, D>> for Point<D> {
    fn from(point: &nalgebra::Point<f32, D>) -> Self {
        Self(point.map(|value| R32::from_inner(value)))
    }
}

impl<const D: usize> From<Point<D>> for [R32; D] {
    fn from(point: Point<D>) -> Self {
        point.into()
    }
}

impl<const D: usize> From<&Point<D>> for [R32; D] {
    fn from(point: &Point<D>) -> Self {
        let mut array: [R32; D] = [R32::default(); D];

        for (element_a, &element_p) in array.iter_mut().zip(point.0.iter()) {
            *element_a = element_p;
        }

        array
    }
}

impl<const D: usize> From<Point<D>> for nalgebra::Point<f32, D> {
    fn from(point: Point<D>) -> Self {
        point.map(|value| value.into_inner())
    }
}

impl<const D: usize> From<&Point<D>> for nalgebra::Point<f32, D> {
    fn from(point: &Point<D>) -> Self {
        point.map(|value| value.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn points_should_have_defined_order() {
        let a = Point::new(0.0, 1.0);
        let b = Point::new(1.0, 0.0);

        assert_eq!(a > b, false);
        assert_eq!(a < b, true);
        assert_eq!(b > a, true);
        assert_eq!(b < a, false);
    }
}
