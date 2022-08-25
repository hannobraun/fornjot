//! Collection of algorithms that are used by the kernel
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.

mod triangulate;

pub mod approx;
pub mod intersect;
pub mod reverse;
pub mod sweep;
pub mod transform;

pub use self::triangulate::triangulate;
