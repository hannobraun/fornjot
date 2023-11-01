//! Operations to update shapes

pub mod build;
pub mod insert;
pub mod join;
pub mod merge;
pub mod reverse;
pub mod split;
pub mod update;

pub use self::update::{
    UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
    UpdateSketch, UpdateSolid,
};
