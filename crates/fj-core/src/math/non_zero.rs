use std::ops::{self, Deref};

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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

impl<T, Rhs> ops::Mul<NonZero<Rhs>> for NonZero<T>
where
    T: ops::Mul<Rhs>,
    T::Output: num_traits::Zero,
{
    type Output = NonZero<T::Output>;

    fn mul(self, rhs: NonZero<Rhs>) -> Self::Output {
        let value = self.into_value().mul(rhs.into_value());

        let Some(non_zero) = NonZero::new(value) else {
            unreachable!(
                "Multiplying two non-zero values must result in a non-zero \
                value."
            );
        };

        non_zero
    }
}

impl<T> ops::Neg for NonZero<T>
where
    T: ops::Neg,
    T::Output: num_traits::Zero,
{
    type Output = NonZero<T::Output>;

    fn neg(self) -> Self::Output {
        let value = self.into_value().neg();

        let Some(non_zero) = NonZero::new(value) else {
            unreachable!(
                "Negating a non-zero value must result in a non-zero value"
            );
        };

        non_zero
    }
}
