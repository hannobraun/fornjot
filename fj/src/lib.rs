pub mod geometry;
pub mod syntax;

pub mod prelude {
    pub use super::syntax::{MakeDifference as _, MakeLinearExtrude as _};
}

mod graphics;
mod input;
mod run;
mod transform;

pub use self::{
    geometry::{Circle, Mesh, Sphere, Triangle3},
    graphics::{Index, Vertex},
    run::run,
};
