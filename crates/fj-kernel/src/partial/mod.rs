//! API for dealing with partially defined objects
//!
//! This module contains types that represent objects that only have some of
//! their data and referenced objects defined. This is useful in the following
//! situations:
//!
//! - Sometimes parts of an object can be inferred. For example, when building a
//!   half-edge that is a line segment, it is enough to provide only two partial
//!   vertices with only their surface coordinates defined. The rest can be
//!   inferred.
//! - Sometimes you need to build an object, but parts of it already exist. For
//!   example, a new half-edge might share a vertex with an existing half-edge.
//!   In such a case you can use the partial object to provide the existing
//!   vertex, then provide or infer other parts as appropriate.
//! - When transforming an object, parts of it might already be transformed. For
//!   example, when transforming a half-edge, each of its vertices references
//!   the same curve as the half-edge does. The partial object API can be used
//!   to avoid transforming the same object multiple times.
//!
//! This module contains two groups of types:
//!
//! - Structs that represent partial objects. For example [`PartialHalfEdge`] is
//!   the partial variant of [`HalfEdge`].
//! - Infrastructure for abstracting over partial objects. See [`Partial`],
//!   [`HasPartial`], and [`MaybePartial`].
//!
//! [`HalfEdge`]: crate::objects::HalfEdge
//!
//! # Implementation Note
//!
//! This API grew out of the [builder API][crate::builder] and is still
//! incomplete. Eventually, it should replace the builder API completely
//! ([#1147]).
//!
//! [#1147]: https://github.com/hannobraun/Fornjot/issues/1147

mod maybe_partial;
mod objects;
mod traits;

pub use self::{
    maybe_partial::MaybePartial,
    objects::{
        curve::PartialCurve,
        edge::{PartialGlobalEdge, PartialHalfEdge},
        vertex::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
    },
    traits::{HasPartial, Partial},
};
