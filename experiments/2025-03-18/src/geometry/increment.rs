use fj_math::{Point, Vector};

/// # The increment of a curve approximation, in curve space
///
/// See [`CurveGeometry::increment`].
#[derive(Clone, Copy)]
pub struct Increment {
    pub inner: Vector<1>,
}

impl Increment {
    pub fn snap_to_multiple(&self, point: Point<1>) -> Point<1> {
        (point / self.inner).floor() * self.inner
    }
}
