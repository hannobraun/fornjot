//! Infrastructure for abstracting over accessing referenced objects

use crate::storage::Handle;

/// Access a single referenced object
///
/// Object types implement this trait for the objects they reference. It can be
/// used by other generic infrastructure to abstract over object access.
///
/// This trait is specifically intended to access single objects, like *the*
/// curve that a vertex references, not *a* half-edge that a cycle references.
pub trait Get<T> {
    /// Access the referenced object
    fn get(&self) -> Handle<T>;
}
