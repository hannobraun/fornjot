//! # Operations to Replace Objects
//!
//! Most objects reference other objects, which can in turn reference even more
//! objects. All of these objects form a graph. Objects are immutable, which
//! means changing one in any way means creating a new version of the object,
//! and hence a new version of the objects that reference it, all the way to the
//! root of the graph.
//!
//! Replace operations replace objects within the object graph, solving a few
//! problems that would otherwise occur here:
//!
//! 1. They take care of *finding* the object. A replace operation for a given
//!    object can be called on any object that references it, directly or
//!    indirectly, so the caller does not need to know which objects reference
//!    the called object directly.
//! 2. They take care of creating new versions of all objects referencing the
//!    replaced object in the whole object graph defined by the object that the
//!    replace operation is called on.
//! 3. They *only* create a new version of an object, if anything has actually
//!    been replaced.
//!
//!
//! ## Structure
//!
//! All replace operations follow the same structure:
//!
//! - They take a reference to the [`Handle`] of the original object that
//!   should be replaced.
//! - Based on the specific replace operations, they take the [`Handle`] of the
//!   replacement, or multiple handles that replace the object. (Depending on
//!   the arity of the reference.)
//! - If the original object is referenced (directly or indirectly) by the
//!   object the operation is called on, it is replaced with the replacement. If
//!   not, nothing happens.
//! - They return an enum that indicates whether an object was actually
//!   replaced. If it was, it contains the [`Handle`] to the new version of the
//!   object the operation was called on. If it wasn't, it contains the original
//!   handle.
//!
//!
//! ## Comparison to Update Operations
//!
//! There is another type of operation, [update operations], which has some
//! conceptual overlap with replace operations. There are some differences
//! though:
//!
//! - Each update operation is only implemented for one type of object
//!   respectively, the one it updates.
//! - Update operations cover changes to attributes that are not references to
//!   other objects.
//! - Update operations cover changes to references that are not replacements,
//!   like adding more references.
//! - Update operations might provide more convenient methods to replace an
//!   object, if the object they are implemented on references only one such
//!   object. In such a case, the update operation does not need to take a
//!   reference to the object being updated, while the respective replace
//!   operation still does.
//!
//!
//! ## Implementation Notes
//!
//! Only a few replace operations are implemented so far. More can be added, as
//! the need arises.
//!
//! As of this writing, replace operations are generally implemented in the most
//! simple and naive way possible: Iterating over all referenced objects and
//! calling the replace operation recursively. This might have performance
//! implications for large object graphs.
//!
//! There are some update operations that are straight-up redundant with what
//! replace operations are doing. Some of the methods even have the same names.
//! Those haven't been removed yet, as update operations generally require a
//! reference to a bare object, while replace operations require a `Handle`.
//! There are some open questions about how operations in general should deal
//! with objects being inserted or not, so it's probably not worth addressing
//! this before doing a general revamp of how operations deal with inserting.
//!
//!
//! [`Handle`]: crate::storage::Handle
//! [update operations]: crate::operations::update

mod curve;
mod half_edge;
mod vertex;

pub use self::{
    curve::ReplaceCurve, half_edge::ReplaceHalfEdge, vertex::ReplaceVertex,
};

use crate::storage::Handle;

/// The output of a replace operation
///
/// See [module documentation] for more information.
///
/// [module documentation]: self
pub enum ReplaceOutput<T> {
    /// The original object that the replace operation was called on
    ///
    /// If this variant is returned, the object to be replaced was not
    /// referenced, and no replacement happened.
    Original(Handle<T>),

    /// The updated version of the object that the operation was called on
    ///
    /// If this variant is returned, a replacement happened, and this is the new
    /// version of the object that reflects that.
    Updated(Handle<T>),
}

impl<T> ReplaceOutput<T> {
    /// Indicate whether the original object was updated
    pub fn was_updated(&self) -> bool {
        matches!(self, ReplaceOutput::Updated(_))
    }

    /// Convert `self` into a `T`, regardless of variant
    pub fn into_inner(self) -> Handle<T> {
        match self {
            ReplaceOutput::Original(inner) => inner,
            ReplaceOutput::Updated(inner) => inner,
        }
    }
}
