//! Fornjot modeling library
//!
//! The purpose of this library is to support Fornjot models, which are just
//! Rust libraries. Models depend on this library and use the primitives defined
//! here to define a CAD model.
//!
//! To actually display the CAD model, or export it to another file format, you
//! need the Fornjot app. Please refer to the [Fornjot repository] for usage
//! examples.
//!
//! [Fornjot repository]: https://github.com/hannobraun/Fornjot

pub mod syntax;

mod shape_2d;
mod shape_3d;

pub use self::{shape_2d::*, shape_3d::*};

/// A shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape {
    /// A 2D shape
    Shape2d(Shape2d),

    /// A 3D shape
    Shape3d(Shape3d),
}
