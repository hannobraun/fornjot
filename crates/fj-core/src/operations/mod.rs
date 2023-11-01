//! Operations to update shapes

pub mod build;
pub mod insert;
mod join;
mod merge;
mod reverse;
mod split;
mod update;

pub use self::{
    insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
    join::JoinCycle,
    merge::Merge,
    reverse::Reverse,
    split::SplitHalfEdge,
    update::{
        UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        UpdateSketch, UpdateSolid,
    },
};
