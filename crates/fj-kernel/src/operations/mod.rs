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
        solid::{BuildSolid, Tetrahedron},
        surface::BuildSurface,
    },
    insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
    join::JoinCycle,
    update::{
        UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateShell, UpdateSolid,
    },
};
