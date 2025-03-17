//! # Generic math types
//!
//! I'm not go document the types in here any further. It's just regular old
//! math, nothing special.

mod bivector;
mod plane;
mod point;
mod scalar;
mod vector;

pub use self::{
    bivector::Bivector, plane::Plane, point::Point, scalar::Scalar,
    vector::Vector,
};
