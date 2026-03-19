//! # Topological primitives to represent shapes
//!
//! See [`Topology`], which is the main entry point to this module.

mod primitives;
mod store;
mod topology;

pub use self::{
    primitives::{Edge, Face, HalfEdge, HalfFace, Orientation, Solid, Vertex},
    store::{Handle, Store},
    topology::Topology,
};
