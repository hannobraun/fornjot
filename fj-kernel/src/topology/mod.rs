//! Topological objects
//!
//! # Equality
//!
//! Equality of topological objects is defined in terms of the geometry they
//! refer to. That means two topological objects that refer to identical
//! geometry are considered equal, even if they contain [`Handle`]s that refer
//! to objects in different [`Shape`] instances.
//!
//! This is different from the equality of [`Handle`], which follows a strict
//! definition of identity. Two [`Handle`]s are only considered equal, if they
//! refer to objects in the same memory location.

mod edges;
mod faces;
mod vertices;

pub use self::{
    edges::{Cycle, Edge},
    faces::Face,
    vertices::Vertex,
};
