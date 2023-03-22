//! API for building objects

// These are new-style builders that build on top of the partial object API.
mod cycle;
mod edge;
mod face;
mod shell;
mod surface;

pub use self::{
    cycle::CycleBuilder, edge::HalfEdgeBuilder, face::FaceBuilder,
    shell::ShellBuilder, surface::SurfaceBuilder,
};
