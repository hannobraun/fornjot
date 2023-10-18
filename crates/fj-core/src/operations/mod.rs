//! Operations to update shapes

mod build;
mod insert;
mod join;
mod merge;
mod reverse;
mod update;

pub use self::{
    build::{
        cycle::BuildCycle,
        edge::BuildHalfEdge,
        face::{BuildFace, Polygon},
        region::BuildRegion,
        shell::{BuildShell, TetrahedronShell},
        sketch::BuildSketch,
        solid::{BuildSolid, Tetrahedron},
        surface::BuildSurface,
    },
    insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
    join::cycle::JoinCycle,
    merge::Merge,
    reverse::Reverse,
    update::{
        cycle::UpdateCycle, edge::UpdateEdge, face::UpdateFace,
        region::UpdateRegion, shell::UpdateShell, sketch::UpdateSketch,
        solid::UpdateSolid,
    },
};
