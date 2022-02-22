use std::{cmp, f64::consts::PI, fmt, hash::Hash, ops};

use decorum::R64;

/// A rational, finite scalar value
///
/// This is a wrapper around `f64`. On construction, it checks that the `f64`
/// value is neither infinite nor NaN. This allows `Scalar` to provide
/// implementations of [`Eq`], [`Ord`], and [`Hash`], enabling `Scalar` (and
/// types built on top of it), to be used as keys in hash maps, hash sets, and
/// similar types.
///
/// # Failing `From`/`Into` implementations
///
/// Please note that the [`From`]/[`Into`] implementation that convert floating
/// point numbers into `Scalar` can panic. These conversions call
/// [`Scalar::from_f64`] internally and panic under the same conditions.
///
/// This explicitly goes against the mandate of [`From`]/[`Into`], whose
/// documentation mandate that implementations must not fail. This is a
/// deliberate design decision. The intended use case of `Scalar` is math code
/// that considers non-finite floating point values a bug, not a recoverable
/// error.
///
/// For this use case, having easy conversions available is an advantage, and
/// explicit `unwrap`/`expect` calls would add nothing. In addition, the mandate
/// not to fail is not motivated in any way, in the [`From`]/[`Into`]
/// documentation.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar(f64);

impl Scalar {
    /// The `Scalar` instance that represents zero
    pub const ZERO: Self = Self(0.);

    /// The `Scalar` instance that represents one
    pub const ONE: Self = Self(1.);

    /// The `Scalar` instance that represents two
    pub const TWO: Self = Self(2.);

    /// The largest `Scalar` value
    pub const MAX: Self = Self(f64::MAX);

    /// The `Scalar` instance that represents pi
    pub const PI: Self = Self(PI);

    /// Construct a `Scalar` from an `f64`
    ///
    /// Panics, if `scalar` is infinite or NaN.
    pub fn from_f64(scalar: f64) -> Self {
        if scalar.is_finite() {
            // `scalar` is neither infinite, nor NaN
            Self(scalar)
        } else {
            panic!("Invalid scalar value: {scalar}");
        }
    }

    /// Construct a `Scalar` from a `u64`
    pub fn from_u64(scalar: u64) -> Self {
        Self::from_f64(scalar as f64)
    }

    /// Convert the scalar into an `f32`
    pub fn into_f32(self) -> f32 {
        self.0 as f32
    }

    /// Convert the scalar into an `f64`
    pub fn into_f64(self) -> f64 {
        self.0
    }

    /// Convert the scalar into a `u64`
    pub fn into_u64(self) -> u64 {
        self.0 as u64
    }

    /// Compute the absolute value of the scalar
    pub fn abs(self) -> Scalar {
        self.0.abs().into()
    }

    /// Compute the maximum of this and another scalar
    pub fn max(self, other: Self) -> Self {
        self.0.max(other.0).into()
    }

    /// Compute the smallest integer larger than or equal to this scalar
    pub fn ceil(self) -> Self {
        self.0.ceil().into()
    }

    /// Round the scalar
    pub fn round(self) -> Self {
        self.0.round().into()
    }

    /// Compute the cosine
    pub fn cos(self) -> Self {
        self.0.cos().into()
    }

    /// Compute sine and cosine
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (sin.into(), cos.into())
    }

    /// Compute the arccosine
    pub fn acos(self) -> Self {
        self.0.acos().into()
    }

    /// Compute the four-quadrant arctangent
    pub fn atan2(self, other: Self) -> Self {
        self.0.atan2(other.0).into()
    }
}

impl Eq for Scalar {}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Should never panic, as `from_f64` checks that the wrapped value is
        // finite.
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for Scalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        R64::from_inner(self.0).hash(state);
    }
}

impl From<f32> for Scalar {
    fn from(scalar: f32) -> Self {
        Self::from_f64(scalar as f64)
    }
}

impl From<f64> for Scalar {
    fn from(scalar: f64) -> Self {
        Self::from_f64(scalar)
    }
}

impl From<Scalar> for f64 {
    fn from(scalar: Scalar) -> Self {
        scalar.into_f64()
    }
}

impl ops::Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}

impl ops::Add<Self> for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}

impl ops::Sub<Self> for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}

impl ops::Mul<Scalar> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}

impl ops::Mul<f64> for Scalar {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0.mul(rhs).into()
    }
}

impl ops::Div<Scalar> for Scalar {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        self.0.div(rhs.0).into()
    }
}

impl ops::Div<f64> for Scalar {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.0.div(rhs).into()
    }
}

impl ops::Rem<Self> for Scalar {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.0.rem(rhs.0).into()
    }
}

impl num_traits::Zero for Scalar {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl num_traits::One for Scalar {
    fn one() -> Self {
        Self::ONE
    }
}

impl num_traits::Num for Scalar {
    type FromStrRadixErr = <f64 as num_traits::Num>::FromStrRadixErr;

    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        f64::from_str_radix(str, radix).map(Scalar::from_f64)
    }
}

impl num_traits::Signed for Scalar {
    fn abs(&self) -> Self {
        self.0.abs().into()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        <f64 as num_traits::Signed>::abs_sub(&self.0, &other.0).into()
    }

    fn signum(&self) -> Self {
        <f64 as num_traits::Signed>::signum(&self.0).into()
    }

    fn is_positive(&self) -> bool {
        <f64 as num_traits::Signed>::is_positive(&self.0)
    }

    fn is_negative(&self) -> bool {
        <f64 as num_traits::Signed>::is_negative(&self.0)
    }
}

impl fmt::Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl approx::AbsDiffEq for Scalar {
    type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
