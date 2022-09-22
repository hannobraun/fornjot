//! Partial objects
//!
//! This module contains type that represent partial objects. This is useful
//! when building objects, as it's often possible to provide just some the data
//! they own or objects they reference, while computing the rest.
//!
//! More generally speaking, there are situations where different parts of a new
//! objects are available at different times, and provided from different
//! places. Partial objects can be used to represent such partially constructed
//! objects whenever that is required.
//!
//! The API for partial objects follows a specific style:
//!
//! - Partial objects are structs with fields that mirror the fields of the full
//!   object structs, but all fields are optional.
//! - Partial object structs implement [`Default`], but a `partial` method is
//!   also available on the respective full object struct, as a perhaps more
//!   convenient and readable way to construct a partial object.
//! - Partial object structs have `with_*` methods to provide values for each of
//!   their fields.
//! - Partial object structs may have other methods with prefixes like `as_*`,
//!   `from_*`, or similar, if one or more of their fields can be initialized by
//!   providing alternative data.
//! - Partial object structs have a `build` method to build a full object.
//! - All `with_*`, `as_*`, and `build` methods can be chained, to provide a
//!   convenient API.

mod curve;
mod vertex;

pub use self::{
    curve::{CurveBuilder, GlobalCurveBuilder},
    vertex::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
};
