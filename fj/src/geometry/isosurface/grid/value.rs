use std::fmt;

use nalgebra::Point;

use crate::geometry::util::DebugPoint;

use super::Index;

#[derive(Clone, Copy, PartialEq)]
pub struct Value {
    pub index: Index,
    pub point: Point<f32, 3>,
    pub value: f32,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ({:.2})", DebugPoint(self.point), self.value)
    }
}
