//! # Tools for approximating shapes

mod circle;
mod tolerance;

pub use self::{
    circle::CircleApproxParams,
    tolerance::{InvalidTolerance, Tolerance},
};
