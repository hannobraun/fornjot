//! Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. An
//! object can be simple and just contain data (like [`Vertex`], for example),
//! or they can be quite complex and refer to other objects (which is actually
//! most of them).
//!
//! # Object Identity vs Object Equality
//!
//! Two objects are *equal*, if they contain the same data. For example, two
//! instances of [`Vertex`] are equal, if they have the same position. This
//! doesn't mean those objects are *identical*. They might have been created by
//! different pieces of code. Or maybe by the same piece of code, but at
//! different times, maybe even based on different inputs.
//!
//! This distinction is relevant, because non-identical objects that are
//! *supposed* to be equal can end up being equal, if they are created based on
//! simple input data (as you might have in a unit test). But they might end up
//! slightly different, if they are created based on complex input data (as you
//! might have in the real world).
//!
//! ## Validation Must Use Identity
//!
//! To prevent such situations, where everything looked fine during development,
//! but you end up with a bug in production, any validation code that compares
//! objects and expects them to be the same, must do that comparison based on
//! identity, not equality. That way, this problem can never happen, because we
//! never expect non-identical objects to be the same.
//!
//! ## How Identity Works
//!
//! We can exactly determine the identity of an object, thanks to [centralized
//! object storage][`Objects`]. If objects are created at different times,
//! potentially by different code, they end up being stored at different memory
//! locations, regardless of their (non-)equality.
//!
//! If you have two [`Handle`]s, you can compare the identity of the objects
//! they point to using the `id` method.
//!
//! [`Handle`]: crate::storage::Handle

mod kinds;
mod object;
mod set;
mod stores;

pub use self::{
    kinds::{
        cycle::{Cycle, HalfEdgesOfCycle},
        edge::{GlobalEdge, HalfEdge},
        face::{Face, FaceSet, Handedness},
        shell::Shell,
        sketch::Sketch,
        solid::Solid,
        surface::Surface,
        vertex::Vertex,
    },
    object::{Bare, BehindHandle, Form, Object, WithHandle},
    set::ObjectSet,
    stores::{Objects, Surfaces},
};
