//! # Fornjot Modeling Library
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! The purpose of this library is to support Fornjot models, which are just
//! Rust libraries. Models depend on this library and use the primitives defined
//! here to define a CAD model. Together with the Fornjot application, this
//! library forms the part of Fornjot that is relevant to end users.
//!
//! To display the created CAD model, or export it to another file format, you
//! need the Fornjot application. Please refer to the [Fornjot repository] for
//! usage examples.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [Fornjot repository]: https://github.com/hannobraun/Fornjot

#![deny(missing_docs)]

pub mod syntax;

mod shape_2d;
mod shape_3d;

pub use self::{shape_2d::*, shape_3d::*};

/// A shape
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Shape {
    /// A 2D shape
    Shape2d(Shape2d),

    /// A 3D shape
    Shape3d(Shape3d),
}
