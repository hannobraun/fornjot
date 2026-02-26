//! # Tools for approximating shapes

use crate::math::Scalar;

/// # A tolerance value
///
/// A tolerance value is used during approximation. It defines the maximum
/// allowed deviation of the approximation from the actual shape.
///
/// The `Tolerance` type enforces that the tolerance value is always larger than
/// zero, which is an attribute that the approximation code relies on.
///
/// ## Failing [`From`]/[`Into`] implementation
///
/// The [`From`]/[`Into`] implementations of tolerance are fallible, which goes
/// against the explicit mandate of those traits, as stated in their
/// documentation.
///
/// A fallible [`Into`] provides a lot of convenience in test code. Since said
/// documentation doesn't provide any actual reasoning for this requirement, I'm
/// feeling free to just ignore it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Tolerance(Scalar);

impl Tolerance {
    /// Construct a `Tolerance` from a [`Scalar`]
    ///
    /// Returns an error, if the passed scalar is not larger than zero.
    pub fn from_scalar(
        scalar: impl Into<Scalar>,
    ) -> Result<Self, InvalidTolerance> {
        let scalar = scalar.into();

        if scalar <= Scalar::ZERO {
            return Err(InvalidTolerance(scalar));
        }

        Ok(Self(scalar))
    }

    /// Return the [`Scalar`] that defines the tolerance
    pub fn inner(&self) -> Scalar {
        self.0
    }
}

impl<S> From<S> for Tolerance
where
    S: Into<Scalar>,
{
    fn from(scalar: S) -> Self {
        Self::from_scalar(scalar)
            .expect("Tried to create `Tolerance` from invalid value")
    }
}

/// Error converting scalar to tolerance
#[derive(Debug, thiserror::Error)]
#[error("Invalid tolerance ({0}); must be above zero")]
pub struct InvalidTolerance(Scalar);
