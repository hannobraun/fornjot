//! Operations to update shapes

mod build;
mod insert;

pub use self::{
    build::{BuildFace, BuildShell, BuildSurface, Tetrahedron, Triangle},
    insert::Insert,
};
