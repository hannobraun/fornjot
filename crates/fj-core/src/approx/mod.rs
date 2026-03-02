//! # Tools for approximating shapes

mod circle;
mod tolerance;

pub use self::{
    circle::CircleApprox,
    tolerance::{InvalidTolerance, Tolerance},
};
