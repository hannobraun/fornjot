mod face_point;

/// Test whether an object or shape contains another
pub trait Contains<T> {
    /// Test whether an object or shape contains another
    ///
    /// Returns `true`, if `self` fully contains `other`, `false` otherwise. A
    /// negative return value could mean that `other` is completely outside of
    /// `self`, or that they intersect.
    fn contains(&self, object: &T) -> bool;
}
