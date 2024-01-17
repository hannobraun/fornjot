//! # Fornjot Interop Types
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library defines types that allow other components of Fornjot to
//! interoperate, without having to depend on each other.
//!
//! [Fornjot]: https://www.fornjot.app/

mod mesh;
mod model;

pub mod ext;

pub use self::{
    mesh::{Color, Index, Mesh, Triangle},
    model::Model,
};
