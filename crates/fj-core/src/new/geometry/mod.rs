//! # Geometrical primitives
//!
//! While [topology][`topology`] defines how parts of a shape relate to each
//! other, geometry defines where those parts of a shape (and thus the shape
//! overall) are located in space.
//!
//! Fornjot approximates geometry and embeds these approximations within the
//! [topological primitives][`topology`]. This module thus plays a supporting
//! role, by helping to create these approximations.
//!
//! [`topology`]: crate::new::topology

mod curve;
mod surface;

pub use self::{
    curve::{Arc, Curve},
    surface::Plane,
};
