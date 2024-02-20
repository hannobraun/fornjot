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
//! [`Handle`]: crate::storage::Handle

mod any_object;
mod is_object;
mod kinds;
mod object_set;
mod stores;

pub use self::{
    any_object::{AboutToBeStored, AnyObject, Bare, Form, Stored},
    is_object::IsObject,
    kinds::{
        curve::Curve,
        cycle::Cycle,
        face::{Face, Handedness},
        half_edge::HalfEdge,
        region::Region,
        shell::Shell,
        sketch::Sketch,
        solid::Solid,
        surface::Surface,
        vertex::Vertex,
    },
    object_set::{ObjectSet, ObjectSetIntoIter, ObjectSetIter},
    stores::{Objects, Surfaces},
};
