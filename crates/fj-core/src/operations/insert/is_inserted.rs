use std::borrow::Borrow;

use crate::storage::Handle;

/// Indicate whether an object has been inserted
///
/// Intended to be used as a type parameter bound for structs that need to track
/// whether their contents have been inserted or not.
pub trait IsInserted {
    /// The type of the object for which the insertion status is tracked
    type T<T>: Borrow<T>;
}

/// Indicate that an object has been inserted
///
/// See [`IsInserted`].
pub struct IsInsertedYes;

impl IsInserted for IsInsertedYes {
    type T<T> = Handle<T>;
}

/// Indicate that an object has not been inserted
///
/// See [`IsInserted`].
pub struct IsInsertedNo;

impl IsInserted for IsInsertedNo {
    type T<T> = T;
}
