//! # The core trait that ties everything together
//!
//! See [`Operation`].

use std::fmt;

use super::{Triangle, Vertex};

/// # An operation
///
/// Provides access to the uniform intermediate representation of operations,
/// which is a triangle mesh.
///
/// This trait is implemented by all operations. Chiefly by the primitive
/// operations, [`Vertex`] and [`Triangle`], but also by
/// [`OpsLog`](crate::geometry::OpsLog) and various types of its supporting
/// infrastructure.
///
/// Even though the geometry representation in this experiment is much more
/// basic than what follow-up experiments are expected to explore, this
/// multitude of implementors is a good sign for the flexibility of this
/// concept.
pub trait Operation: fmt::Display {
    /// # The vertices that are part of the operation's uniform representation
    ///
    /// Many callers won't have to bother with this method, as the vertices are
    /// also available indirectly through [`Operation::triangles`]. But this is
    /// used by the viewer, for example, to render the shape as it is
    /// constructed, vertex by vertex.
    fn vertices(&self, vertices: &mut Vec<Vertex>);

    /// # The triangles that are part of the operation's uniform representation
    fn triangles(&self, triangles: &mut Vec<Triangle>);
}
