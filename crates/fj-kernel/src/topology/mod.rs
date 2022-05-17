//! Topological objects
//!
//! Simplifying a bit, topology is responsible for defining how objects are
//! related, as opposed to geometry, which is responsible for where things are.
//!
//! The types in this module use the types from [`crate::geometry`].
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

mod builder;
mod cycle;
mod edge;
mod face;
mod vertex;

pub use self::{
    builder::{CycleBuilder, EdgeBuilder, FaceBuilder, VertexBuilder},
    cycle::Cycle,
    edge::Edge,
    face::{CyclesInFace, Face},
    vertex::Vertex,
};
