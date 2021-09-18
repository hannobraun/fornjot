#![allow(uncommon_codepoints)]

pub mod geometry;
pub mod model;
pub mod syntax;
pub mod threemf;

pub mod prelude {
    pub use crate::syntax::{Difference as _, Resolution as _, Sweep as _};
}

mod args;
mod draw_config;
mod graphics;
mod input;
mod run;
mod types;
mod util;

pub use self::{
    geometry::{
        attributes::Mesh,
        operations::{Difference, LinearSweep},
        shapes::Cylinder,
    },
    graphics::Vertex,
    model::Model,
    run::{run_mesh, run_model},
    types::Index,
};
