mod shape_2d;
mod shape_3d;

pub use self::{shape_2d::*, shape_3d::*};

/// A shape
#[derive(Debug)]
#[repr(C)]
pub enum Shape {
    Shape2d(Shape2d),
    Shape3d(Shape3d),
}
