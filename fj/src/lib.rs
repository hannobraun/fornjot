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
mod graphics;
mod input;
mod mesh;
mod run;
mod util;

pub use self::{
    geometry::{
        operations::{Difference, Sweep},
        shapes::{Cylinder, MeshMaker, Polygon, Quad, Toroid, Triangle},
    },
    graphics::Vertex,
    mesh::{Index, Mesh},
    model::Model,
    run::{run_mesh, run_model},
};
