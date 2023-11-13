//! # Operations to split objects
//!
//! See [`SplitHalfEdge`], which is currently the only trait in this module, for
//! more information.

mod edge;
mod half_edge;

pub use self::{edge::SplitEdge, half_edge::SplitHalfEdge};
