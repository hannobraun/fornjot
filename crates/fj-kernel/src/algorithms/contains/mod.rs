mod face_point;

/// Test whether an object or shape contains another
///
/// # Implementation Note
///
/// This is basically a more limited version of [`Intersect`]. It probably makes
/// sense to migrate all of this trait's implementations to [`Intersect`] and
/// remove this trait.
///
/// [`Intersect`]: super::intersect::Intersect
pub trait Contains<T> {
    /// Test whether an object or shape contains another
    ///
    /// Returns `true`, if `self` fully contains `other`, `false` otherwise. A
    /// negative return value could mean that `other` is completely outside of
    /// `self`, or that they intersect.
    fn contains(&self, object: &T) -> bool;
}
