//! Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. An
//! object can be simple and just contain data (like, for example, [`Vertex`]),
//! or they can be quite complex and refer to other objects.
//!
//! # Equality
//!
//! Two objects, even if they are distinct and live in different memory
//! locations, are considered equal, if the data they contain and the objects
//! they reference are considered equal.
//!
//! In contrast to that, two [`Handle`]s are considered equal, only if they
//! reference the same object, in the same memory location. This means that two
//! objects can be considered equal, even if the [`Handle`]s they contain are
//! not.
//!
//! Equality is defined like this, two cover two distinct use cases:
//!
//! - If you need to know whether two [`Handle`]s actually refer to the same
//!   object in the same [`Shape`], you can compare the [`Handle`]s.
//! - If you only need to check whether two objects look the same, but don't
//!   care whether they are in the same shape, compare the objects directly.
//!
//! The second use case is common in test code.
//!
//! # Implementation Note
//!
//! The definition of equality, as detailed above, is overly complex. It is
//! necessary though, due to the way the kernel's core data structures work.
//! Each shape's objects are stored in a distinct [`Shape`] structure, even if
//! there is a high amount of redundancy between those shapes.
//!
//! If there was a single, append-only data structure for all objects in a CAD
//! model, in which objects were immutable, there would be no special definition
//! of equality for objects. Unfortunately, nobody has figured out how to make
//! this work yet.
//!
//! [`Handle`]: crate::shape::Handle
//! [`Shape`]: crate::shape::Shape

mod curves;
mod cycle;
mod edge;
mod face;
mod surfaces;
mod vertex;

pub use self::{
    curves::Curve,
    cycle::Cycle,
    edge::{Edge, VerticesOfEdge},
    face::{CyclesInFace, Face},
    surfaces::{Surface, SweptCurve},
    vertex::Vertex,
};
