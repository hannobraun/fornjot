use std::ops::Deref;

/// # Wrapper around a value to guarantee it's not zero
///
/// ## Implementation Note
///
/// This is redundant with [`std::num::NonZero`]. The contents of that have a
/// trait bound on [`std::num::ZeroablePrimitive], which as of Rust 1.93 is
/// experimental, and thus not available to the types we'd like to use it with.
///
/// Once that situation has changed, it might be desirable to replace this type
/// with [`std::num::NonZero`].
pub struct NonZero<T> {
    value: T,
}

impl<T> NonZero<T> {
    /// # Construct a new instance of `NonZero`
    ///
    /// Returns `None`, if the provided value is zero. Returns an instance of
    /// `NonZero` otherwise.
    pub fn new(value: T) -> Option<Self>
    where
        T: num_traits::Zero,
    {
        if value.is_zero() {
            return None;
        }

        Some(Self { value })
    }

    /// # Convert this instance of `NonZero` into its wrapped value
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> Deref for NonZero<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
