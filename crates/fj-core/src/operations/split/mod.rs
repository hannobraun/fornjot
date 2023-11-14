//! # Operations to split objects
//!
//! Splitting means removing an object, replacing it with to new ones that fill
//! the same space. This often makes sense, when you want to modify only part of
//! an object. In such a case, you can split off the part you want to modify,
//! leaving the rest unchanged.

mod edge;
mod half_edge;

pub use self::{edge::SplitEdge, half_edge::SplitHalfEdge};
