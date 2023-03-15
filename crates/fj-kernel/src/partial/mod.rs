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
        shell::PartialShell, sketch::PartialSketch, solid::PartialSolid,
    },
    traits::{HasPartial, PartialObject},
    wrapper::{FullToPartialCache, Partial},
};

use crate::storage::Handle;

/// Either a full or a partial object
///
/// # Implementation Note
///
/// This enum temporarily exists to aid in the transition towards a unified
/// object system. Issue:
/// <https://github.com/hannobraun/Fornjot/issues/1570>
#[derive(Clone, Debug)]
pub enum FullOrPartial<T: HasPartial + 'static> {
    /// A full object
    Full(Handle<T>),

    /// A partial object
    Partial(Partial<T>),
}

impl<T: HasPartial> From<Handle<T>> for FullOrPartial<T> {
    fn from(object: Handle<T>) -> Self {
        Self::Full(object)
    }
}

impl<T: HasPartial> From<Partial<T>> for FullOrPartial<T> {
    fn from(object: Partial<T>) -> Self {
        Self::Partial(object)
    }
}
