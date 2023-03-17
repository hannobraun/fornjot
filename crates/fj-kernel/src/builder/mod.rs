//! API for building objects

// These are new-style builders that build on top of the partial object API.
mod cycle;
mod edge;

pub use self::{
    cycle::{CycleBuilder, CycleBuilder2},
    edge::HalfEdgeBuilder,
};
