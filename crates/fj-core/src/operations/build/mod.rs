//! # Operations to build objects
//!
//! All object structs have constructors, but building even simple shapes by
//! just using those is pretty difficult. This is why the traits in this module
//! exist. They build on top of those low-level constructors, providing a more
//! rich set of operations to build geometry.
//!
//!
//! ## Wrapper Structs
//!
//! Many of the the trait methods return the object that is being built
//! directly, but others return a wrapper struct. An example of this is
//! [`BuildFace::triangle`], which returns [`Polygon`] instead of returning the
//! face directly.
//!
//! These wrapper structs are designed to provide convenient access not only to
//! the top-level object itself, but also to the other objects that make up its
//! components.

mod cycle;
mod face;
mod half_edge;
mod region;
mod shell;
mod sketch;
mod solid;
mod surface;

pub use self::{
    cycle::BuildCycle,
    face::{BuildFace, Polygon},
    half_edge::BuildHalfEdge,
    region::BuildRegion,
    shell::{BuildShell, TetrahedronShell},
    sketch::BuildSketch,
    solid::{BuildSolid, Tetrahedron},
    surface::BuildSurface,
};
