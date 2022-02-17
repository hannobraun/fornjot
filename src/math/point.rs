use std::ops;

use super::Vector;

pub type Point<const D: usize> = nalgebra::Point<f64, D>;

impl<const D: usize> ops::Add<Vector<D>> for Point<D> {
    type Output = Self;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        self.add(rhs.to_na())
    }
}
