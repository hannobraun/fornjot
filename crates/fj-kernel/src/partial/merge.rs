/// Trait for merging partial objects
///
/// Implemented for all partial objects themselves, and also some related types
/// that partial objects usually contain.
pub trait MergeWith: Sized {
    /// Merge this object with another
    ///
    /// # Panics
    ///
    /// Merging two objects that cannot be merged is considered a programmer
    /// error and will result in a panic.
    fn merge_with(self, other: impl Into<Self>) -> Self;
}
