//! Operations to update shapes

mod build;
mod insert;
mod join;
mod update;

pub use self::{
    build::{
        BuildCycle, BuildFace, BuildHalfEdge, BuildShell, BuildSurface,
        Polygon, Tetrahedron,
    },
    insert::Insert,
    join::JoinCycle,
    update::{UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateShell},
};
