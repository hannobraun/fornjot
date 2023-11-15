//! # Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. Objects
//! can reference each other, forming a directed acyclic graph (DAG).
//!
//! There are two top-level objects ([`Sketch`] and [`Solid`]) which represent
//! whole shapes (2D and 3D, respectively), while all other objects are
//! referenced (directly or indirectly) by these top-level objects.
//!
//! All objects are stored in centralized storage (see [`Objects`]) and referred
//! to through a [`Handle`].
//!
//!
//! ## Object Equality vs Object Identity
//!
//! Most objects have [`Eq`]/[`PartialEq`] implementations that can be used to
//! determine equality. Those implementations are derived, meaning two objects
//! are equal, if all of their fields are equal. This can be used to compare
//! objects structurally. [`Handle`]'s own [`Eq`]/[`PartialEq`] implementations
//! defer to those of the object it references.
//!
//! However, that two objects are *equal* does not mean they are *identical*.
//! See [`Handle`]'s documentation for an explanation of this distinction.
//!
//! This distinction is relevant, because non-identical objects that are
//! *supposed* to be equal can end up being equal, if they are created based on
//! simple input data (as you might have in a unit test). But they might end up
//! slightly different, if they are created based on complex input data (as you
//! might have in the real world). This situation would most likely result in a
//! bug that is not easily caught in testing.
//!
//! ### Validation Must Use Identity
//!
//! To prevent such situations, where everything looked fine during development,
//! but you end up with a bug in production, any validation code that compares
//! objects and expects them to be the same, must do that comparison based on
//! identity, not equality. That way, this problem can never happen, because we
//! never expect non-identical objects to be the same.
//!
//! [`Handle`]: crate::storage::Handle

mod handles;
mod kinds;
mod object;
mod stores;

pub use self::{
    handles::Handles,
    kinds::{
        curve::Curve,
        cycle::Cycle,
        edge::HalfEdge,
        face::{Face, Handedness},
        region::Region,
        shell::Shell,
        sketch::Sketch,
        solid::Solid,
        surface::Surface,
        vertex::Vertex,
    },
    object::{Bare, BehindHandle, Form, Object, WithHandle},
    stores::{Objects, Surfaces},
};
