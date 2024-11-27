//! Core geometry representation
//!
//! This is the core of what this experiment is about! The goal was to build an
//! interactive core around a very simple geometry representation (a triangle
//! mesh).
//!
//! While the geometry representation is very basic, I actually expect that
//! follow-up experiments will add more layers on top to structure it (thereby
//! enriching it with topographical information), rather than replace it with
//! something inherently more advanced (like NURBS or whatever).
//!
//! The idea here is that the triangle mesh works as a uniform intermediate
//! representation for geometry (as I've already been working towards with the
//! mainline Fornjot code), that any necessary algorithms can be built around
//! of. But working that out is not the subject of this experiment.
//!
//! If you're interested in the details, I suggest you start with [`OpsLog`],
//! which is the entry point to this API, and work your way down from there.

mod operation;
mod ops_log;
mod primitives;

pub use self::{
    operation::Operation,
    ops_log::OpsLog,
    primitives::{Triangle, Vertex},
};
