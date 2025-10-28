use fj_math::{Point, Vector};

#[derive(Clone, Copy)]
pub struct Increment<const D: usize> {
    pub inner: Vector<D>,
}

impl Increment<1> {
    pub fn snap_to_multiple(&self, point: Point<1>) -> Point<1> {
        (point / self.inner).floor() * self.inner
    }
}
