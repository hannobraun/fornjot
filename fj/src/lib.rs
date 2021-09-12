#![allow(uncommon_codepoints)]

pub mod geometry;
pub mod mesh;
pub mod model;
pub mod syntax;
pub mod threemf;

pub mod prelude {
    pub use crate::syntax::{Difference as _, Resolution as _, Sweep as _};
}

mod args;
mod graphics;
mod input;
mod render_config;
mod run;
mod types;
mod util;

pub use self::{
    geometry::{
        operations::{Difference, Sweep},
        shapes::Cylinder,
    },
    graphics::Vertex,
    mesh::Mesh,
    model::Model,
    run::{run_mesh, run_model},
    types::Index,
};
