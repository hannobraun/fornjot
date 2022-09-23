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
    curve::{PartialCurve, PartialGlobalCurve},
    vertex::{PartialGlobalVertex, PartialSurfaceVertex, PartialVertex},
};

use crate::{
    objects::{Curve, GlobalCurve, GlobalVertex, SurfaceVertex, Vertex},
    stores::{Handle, Stores},
};

/// Either a partial object or a full one
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum MaybePartial<T: HasPartialForm> {
    /// A full object
    Full(T),

    /// A partial object
    Partial(T::PartialForm),
}

impl<T: HasPartialForm> MaybePartial<T> {
    /// Return the full object, either directly or by building it
    pub fn into_full(self, stores: &Stores) -> T {
        match self {
            Self::Partial(partial) => T::from_partial(partial, stores),
            Self::Full(full) => full,
        }
    }

    /// Return the partial object, either directly or via conversion
    pub fn into_partial(self) -> T::PartialForm {
        match self {
            Self::Partial(partial) => partial,
            Self::Full(full) => full.into(),
        }
    }
}

/// Implemented for types that are partial objects
///
/// # Implementation Note
///
/// It would be nicer to require a conversion from `&Self` into the partial
/// form, but I think we need a `where` clause on the associated type to specify
/// that, which is unstable. It should become stable soon though, together with
/// generic associated types:
/// <https://github.com/rust-lang/rust/issues/44265>
pub trait HasPartialForm: Into<Self::PartialForm> {
    /// The full version of this partial object
    type PartialForm;

    /// Build a full object from the partial object
    fn from_partial(partial: Self::PartialForm, stores: &Stores) -> Self;
}

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartialForm for $full {
                type PartialForm = $partial;

                fn from_partial(partial: Self::PartialForm, stores: &Stores)
                    -> Self
                {
                    partial.build(stores)
                }
            }

            impl From<$full> for MaybePartial<$full> {
                fn from(full: $full) -> Self {
                    Self::Full(full)
                }
            }

            impl From<$partial> for MaybePartial<$full> {
                fn from(partial: $partial) -> Self {
                    Self::Partial(partial)
                }
            }
        )*
    };
}

impl_traits!(
    Curve, PartialCurve;
    GlobalVertex, PartialGlobalVertex;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;

    Handle<GlobalCurve>, PartialGlobalCurve;
);
