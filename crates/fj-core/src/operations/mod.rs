//! Operations to update shapes

mod build;
mod insert;
mod join;
mod merge;
mod reverse;
mod split;
mod update;

pub use self::{
    build::{
        BuildCycle, BuildFace, BuildHalfEdge, BuildRegion, BuildShell,
        BuildSketch, BuildSolid, BuildSurface, Polygon, Tetrahedron,
        TetrahedronShell,
    },
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
