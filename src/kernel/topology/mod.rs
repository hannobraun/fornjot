//! Topological objects
//!
//! # Equality
//!
//! Equality of topological objects is defined in terms of the geometry they
//! refer to. That means two topological objects that refer to identical
//! geometry are considered equal, even if they contain [`Handle`]s that refer
//! to objects in different [`Shape`] instances.

pub mod edges;
pub mod faces;
pub mod vertices;
