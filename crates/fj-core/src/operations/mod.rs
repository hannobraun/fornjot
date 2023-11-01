//! Operations to update shapes

pub mod build;
pub mod insert;
pub mod join;
pub mod merge;
pub mod reverse;
mod split;
mod update;

pub use self::{
    split::SplitHalfEdge,
    update::{
        UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        UpdateSketch, UpdateSolid,
    },
};
