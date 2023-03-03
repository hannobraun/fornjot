//! Partially defined objects
//!
//! This module contains types that mirror the full object types from
//! [`crate::objects`], only the types from this module can be defined only
//! partially, with the non-defined parts being inferred when a full object is
//! constructed.
//!
//! # Implementation Note
//!
//! This API was created as a replacement for the [original partial object
//! API][crate::partial]. This is still a work in progress.

mod objects;
mod traits;
mod wrapper;

pub use self::{
    objects::{
        curve::MaybeCurve, cycle::PartialCycle, edge::PartialHalfEdge,
        face::PartialFace, shell::PartialShell, sketch::PartialSketch,
        solid::PartialSolid, surface::PartialSurface, vertex::PartialVertex,
    },
    traits::{HasPartial, PartialObject},
    wrapper::{FullToPartialCache, Partial},
};
