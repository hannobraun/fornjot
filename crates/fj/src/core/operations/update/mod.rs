//! Operations to update objects

mod cycle;
mod face;
mod half_edge;
mod region;
mod shell;
mod sketch;
mod solid;

pub use self::{
    cycle::UpdateCycle, face::UpdateFace, half_edge::UpdateHalfEdge,
    region::UpdateRegion, shell::UpdateShell, sketch::UpdateSketch,
    solid::UpdateSolid,
};
