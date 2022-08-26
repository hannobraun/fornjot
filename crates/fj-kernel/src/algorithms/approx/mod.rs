//! Approximation of objects

mod curve;
mod cycle;
mod edge;
mod face;
mod local;
mod tolerance;

pub use self::{
    cycle::CycleApprox,
    face::FaceApprox,
    local::{Local, LocalForm},
    tolerance::{InvalidTolerance, Tolerance},
};
