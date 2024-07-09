//! Queries about objects
//!
//! Objects have methods that provide access to anything that the object itself
//! has direct access to. However, not all potentially interesting information
//! can be accessed that way. An example are the bounding vertices of an edge:
//! `Edge` only stores its starting vertex, so you need a `Cycle` to get both
//! vertices.
//!
//! This module provides traits express such non-trivial queries, and implements
//! them for various objects that have the information to answer the query.

mod all_half_edges_with_surface;
mod bounding_vertices_of_half_edge;
mod sibling_of_half_edge;

pub use self::{
    all_half_edges_with_surface::AllHalfEdgesWithSurface,
    bounding_vertices_of_half_edge::BoundingVerticesOfHalfEdge,
    sibling_of_half_edge::{Sibling, SiblingOfHalfEdge},
};
