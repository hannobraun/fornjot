use crate::storage::Handle;

/// Recursively replace a (partial) object referenced by another partial object
pub trait Replace<T> {
    /// Recursively replace the referenced object
    fn replace(&mut self, object: Handle<T>) -> &mut Self;
}
