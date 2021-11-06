#![allow(uncommon_codepoints)]

pub mod geometry;
pub mod math;
pub mod model;
pub mod syntax;

pub mod prelude {
    pub use crate::syntax::{
        Difference as _, Resolution as _, Rotate as _, Sweep as _,
        Translate as _,
    };
}

mod args;
mod draw_config;
mod graphics;
mod input;
mod mesh;
mod run;
mod types;
mod util;

pub use self::{
    geometry::{
        operations::{Difference, Sweep},
        shapes::{Cylinder, Mesh, MeshMaker, Polygon, Quad, Toroid, Triangle},
    },
    graphics::Vertex,
    model::Model,
    run::{run_mesh, run_model},
    types::Index,
};
