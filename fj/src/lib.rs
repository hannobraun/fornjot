pub mod geometry;

pub mod prelude {
    pub use super::geometry::operations::difference::MakeDifference as _;
}

mod graphics;
mod input;
mod run;
mod transform;

pub use self::{
    geometry::{Circle, Mesh, Triangle},
    graphics::{Index, Vertex},
    run::run,
};
