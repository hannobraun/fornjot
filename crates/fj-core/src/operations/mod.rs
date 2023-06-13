//! Operations to update shapes

mod build;
mod insert;
mod join;
mod update;

pub use self::{
    build::{
        cycle::BuildCycle,
        edge::BuildHalfEdge,
        face::{BuildFace, Polygon},
        shell::{BuildShell, TetrahedronShell},
        sketch::BuildSketch,
        solid::{BuildSolid, Tetrahedron},
        surface::BuildSurface,
    },
    insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
    join::cycle::JoinCycle,
    update::{
        cycle::UpdateCycle, edge::UpdateHalfEdge, face::UpdateFace,
        region::UpdateRegion, shell::UpdateShell, sketch::UpdateSketch,
        solid::UpdateSolid,
    },
};
