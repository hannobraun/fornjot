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

impl<T> MergeWith for Option<T>
where
    T: PartialEq,
{
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        if self == other {
            return self;
        }

        // We know that `self != other`, or we wouldn't have made it here.
        if self.is_some() && other.is_some() {
            // It would be great if we could optionally merge the two values
            // recursively, if they support that, but that requires
            // `specialization`:
            // https://doc.rust-lang.org/nightly/unstable-book/language-features/specialization.html
            //
            // Or maybe `min_specialization`:
            // https://doc.rust-lang.org/nightly/unstable-book/language-features/min-specialization.html
            //
            // Basically, we'd have one default implementation for all types,
            // and a specialized one for `T: MergeWith`.
            panic!("Can't merge two `Option`s that are both `Some`")
        }

        self.xor(other)
    }
}
