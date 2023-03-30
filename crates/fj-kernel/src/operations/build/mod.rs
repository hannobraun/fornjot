mod edge;
mod face;
mod shell;
mod surface;

pub use self::{
    edge::BuildHalfEdge,
    face::{BuildFace, Triangle},
    shell::{BuildShell, Tetrahedron},
    surface::BuildSurface,
};
