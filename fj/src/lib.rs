pub mod geometry;
pub mod syntax;

pub mod prelude {
    pub use super::syntax::{Difference as _, LinearExtrude as _};
}

mod graphics;
mod input;
mod run;
mod transform;

pub use self::{
    geometry::shapes::{Circle, Cylinder, Mesh, Sphere, Triangle3},
    graphics::{Index, Vertex},
    run::run,
};
