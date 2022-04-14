//! Geometry objects
//!
//! Simplifying a bit, geometry is responsible for where things are, but now how
//! they are related. The types in this module are referred to by the types in
//! [`crate::topology`], which are responsible for defining how objects are
//! related.

mod curves;
mod points;
mod surfaces;

pub use self::{
    curves::{Arc, Curve, Line},
    points::Point,
    surfaces::{Surface, SweptCurve},
};
