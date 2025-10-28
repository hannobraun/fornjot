use fj_math::{Point, Vector};

#[derive(Clone, Copy)]
pub struct Increment<const D: usize> {
    pub inner: Vector<D>,
}

impl<const D: usize> Increment<D> {
    pub fn snap_to_multiple(&self, point: Point<D>) -> Point<D> {
        (point / self.inner).floor() * self.inner
    }
}
