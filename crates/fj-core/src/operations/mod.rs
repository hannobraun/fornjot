//! Operations to update shapes

pub mod build;
pub mod insert;
pub mod join;
pub mod merge;
mod reverse;
mod split;
mod update;

pub use self::{
    reverse::Reverse,
    split::SplitHalfEdge,
    update::{
        UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        UpdateSketch, UpdateSolid,
    },
};
