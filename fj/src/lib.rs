pub mod geometry;

pub mod prelude {
    pub use super::geometry::operations::{
        difference::MakeDifference as _, linear_extrude::MakeLinearExtrude as _,
    };
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
