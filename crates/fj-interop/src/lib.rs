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

mod color;
mod tri_mesh;

pub mod ext;

pub use self::{
    color::Color,
    tri_mesh::{Index, TriMesh, Triangle},
};
