pub mod geometry;
pub mod mesh;
pub mod syntax;

pub mod prelude {
    pub use super::syntax::{Difference as _, LinearExtrude as _};
}

mod args;
mod graphics;
mod input;
mod run;

pub use self::{
    geometry::shapes::Cylinder,
    graphics::{Index, Vertex},
    mesh::Mesh,
    run::run,
};
