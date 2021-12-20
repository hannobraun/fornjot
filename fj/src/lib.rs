mod shape_2d;
mod shape_3d;
mod syntax;

pub mod prelude {
    pub use crate::syntax::{
        Rotate as _, Sketch as _, Sweep as _, Translate as _, Union as _,
    };
}

pub use self::{shape_2d::*, shape_3d::*};

/// A shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape {
    Shape2d(Shape2d),
    Shape3d(Shape3d),
}
