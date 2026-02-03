//! # Topological primitives to represent shapes
//!
//! See [`Topology`], which is the main entry point to this module.

mod objects;
mod store;

pub use self::{
    objects::{Face, HalfEdge, Solid, Vertex},
    store::{Index, Store},
};

#[derive(Default)]
pub struct Topology {
    pub faces: Store<Face>,
    pub half_edges: Store<HalfEdge>,
    pub solids: Store<Solid>,
    pub vertices: Store<Vertex>,
}

impl Topology {
    pub fn new() -> Self {
        Self::default()
    }
}
