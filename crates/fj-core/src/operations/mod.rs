//! # Create and modify shapes
//!
//! ## Purpose and Design
//!
//! This module provides functionality to create and modify shapes, generally in
//! the form of extension traits. These extension traits are implemented for the
//! object types that the functionality applies to, making them easily available
//! to the caller, without creating a huge, monolithic API that would clutter up
//! the modules that those object types are defined in.
//!
//! Objects are immutable, so to update one, one must create a new version of
//! the object that incorporates the update. For this reason, the extension
//! traits generally take `&self` and return the newly created object. Trait
//! methods are annotated with `#[must_use]`, to prevent mistakes due to
//! misunderstanding this principle.
//!
//!
//! ### Bare Objects vs Stored Objects
//!
//! Extension traits are mostly implemented for bare object types (i.e. for
//! `Vertex` instead of `Handle<Vertex>`). This makes those operations more
//! flexible, as a `Handle` might not be available, but a reference to the bare
//! object can always be produced from a `Handle`.
//!
//! They also mostly return bare objects, which also provides more flexibility
//! to the user. An inserted object must always be valid, but in a series of
//! operations, any intermediate ones might leave the object in an invalid
//! state. By returning the bare object, the decision when to insert the object
//! is left to the caller.
//!
//! Some operations might deviate from this rule, where it makes sense.
//!
//!
//! ## Implementation Note
//!
//! Not all operation methods take `&self`, and not all are implemented for bare
//! objects first. Where this is done without a clear justification, you can
//! assume that the code in question is outdated. Feel free to open an issue or
//! send a pull request!

pub mod build;
pub mod derive;
pub mod geometry;
pub mod holes;
pub mod insert;
pub mod join;
pub mod merge;
pub mod presentation;
pub mod replace;
pub mod reverse;
pub mod selector;
pub mod split;
pub mod sweep;
pub mod transform;
pub mod update;
