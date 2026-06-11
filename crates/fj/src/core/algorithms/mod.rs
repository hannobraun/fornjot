//! Collection of algorithms that operate on geometry
//!
//! Algorithmic code is collected in this module, to keep other modules focused
//! on their respective purpose.
//!
//! # Implementation Note
//!
//! This module exists in a bit of an in-between state, as some of the things
//! that are still here are probably better placed in the [`operations`] module.
//!
//! [`operations`]: crate::operations

pub mod approx;
pub mod bounding_volume;
pub mod intersect;
pub mod triangulate;
