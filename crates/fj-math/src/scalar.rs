use std::{
    cmp,
    f64::consts::{PI, TAU},
    fmt,
    hash::Hash,
    ops,
};

use decorum::R64;

/// A rational, finite scalar value
///
/// This is a wrapper around `f64`. On construction, it checks that the `f64`
/// value is not NaN. This allows `Scalar` to provide implementations of [`Eq`],
/// [`Ord`], and [`Hash`], enabling `Scalar` (and types built on top of it), to
/// be used as keys in hash maps, hash sets, and similar types.
#[derive(Clone, Copy, Default)]
#[repr(C)]
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

    /// The `Scalar` instance that represents tau
    pub const TAU: Self = Self(TAU);

    /// Construct a `Scalar` from an `f64`
    ///
    /// # Panics
    ///
    /// Panics, if `scalar` is NaN.
    pub fn from_f64(scalar: f64) -> Self {
        if scalar.is_nan() {
            panic!("Invalid scalar value: {scalar}");
        } else {
            Self(scalar)
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

    /// The sign of the scalar
    ///
    /// Return `Scalar::ZERO`, if the scalar is zero, `Scalar::ONE`, if it is
    /// positive, `-Scalar::ONE`, if it is negative.
    pub fn sign(self) -> Scalar {
        if self == Self::ZERO {
            Self::ZERO
        } else {
            Self(self.0.signum())
        }
    }

    /// Compute the absolute value of the scalar
    pub fn abs(self) -> Self {
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

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Should never panic, as `from_f64` checks that the wrapped value is
        // finite.
        self.partial_cmp(other).expect("Invalid `Scalar`")
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for Scalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // To the best of my knowledge, this matches the `PartialEq`
        // implementation.
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

impl ops::AddAssign<Self> for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
        *self = Self::from_f64(self.0);
    }
}

impl ops::Sub<Self> for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}

impl ops::Mul<Self> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}

impl ops::Mul<f64> for Scalar {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0.mul(rhs).into()
    }
}

impl ops::Div<Self> for Scalar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
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
        f64::from_str_radix(str, radix).map(Self::from_f64)
    }
}

impl num_traits::NumCast for Scalar {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::from_f64(<f64 as num_traits::NumCast>::from(n)?))
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

impl num_traits::ToPrimitive for Scalar {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

impl num_traits::Float for Scalar {
    fn nan() -> Self {
        panic!("`Scalar` can not represent NaN")
    }

    fn infinity() -> Self {
        Self::from_f64(f64::infinity())
    }

    fn neg_infinity() -> Self {
        Self::from_f64(f64::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::from_f64(f64::neg_zero())
    }

    fn min_value() -> Self {
        Self::from_f64(f64::min_value())
    }

    fn min_positive_value() -> Self {
        Self::from_f64(f64::min_positive_value())
    }

    fn max_value() -> Self {
        Self::from_f64(f64::max_value())
    }

    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.0.classify()
    }

    fn floor(self) -> Self {
        Self::from_f64(self.0.floor())
    }

    fn ceil(self) -> Self {
        Self::from_f64(self.0.ceil())
    }

    fn round(self) -> Self {
        Self::from_f64(self.0.round())
    }

    fn trunc(self) -> Self {
        Self::from_f64(self.0.trunc())
    }

    fn fract(self) -> Self {
        Self::from_f64(self.0.fract())
    }

    fn abs(self) -> Self {
        Self::from_f64(self.0.abs())
    }

    fn signum(self) -> Self {
        Self::from_f64(self.0.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::from_f64(self.0.mul_add(a.0, b.0))
    }

    fn recip(self) -> Self {
        Self::from_f64(self.0.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self::from_f64(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self::from_f64(self.0.powf(n.0))
    }

    fn sqrt(self) -> Self {
        Self::from_f64(self.0.sqrt())
    }

    fn exp(self) -> Self {
        Self::from_f64(self.0.exp())
    }

    fn exp2(self) -> Self {
        Self::from_f64(self.0.exp2())
    }

    fn ln(self) -> Self {
        Self::from_f64(self.0.ln())
    }

    fn log(self, base: Self) -> Self {
        Self::from_f64(self.0.log(base.0))
    }

    fn log2(self) -> Self {
        Self::from_f64(self.0.log2())
    }

    fn log10(self) -> Self {
        Self::from_f64(self.0.log10())
    }

    fn max(self, other: Self) -> Self {
        Self::from_f64(self.0.max(other.0))
    }

    fn min(self, other: Self) -> Self {
        Self::from_f64(self.0.min(other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        (self - other).abs()
    }

    fn cbrt(self) -> Self {
        Self::from_f64(self.0.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self::from_f64(self.0.hypot(other.0))
    }

    fn sin(self) -> Self {
        Self::from_f64(self.0.sin())
    }

    fn cos(self) -> Self {
        Self::from_f64(self.0.cos())
    }

    fn tan(self) -> Self {
        Self::from_f64(self.0.tan())
    }

    fn asin(self) -> Self {
        Self::from_f64(self.0.asin())
    }

    fn acos(self) -> Self {
        Self::from_f64(self.0.acos())
    }

    fn atan(self) -> Self {
        Self::from_f64(self.0.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self::from_f64(self.0.atan2(other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self::from_f64(sin), Self::from_f64(cos))
    }

    fn exp_m1(self) -> Self {
        Self::from_f64(self.0.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::from_f64(self.0.ln_1p())
    }

    fn sinh(self) -> Self {
        Self::from_f64(self.0.sinh())
    }

    fn cosh(self) -> Self {
        Self::from_f64(self.0.cosh())
    }

    fn tanh(self) -> Self {
        Self::from_f64(self.0.tanh())
    }

    fn asinh(self) -> Self {
        Self::from_f64(self.0.asinh())
    }

    fn acosh(self) -> Self {
        Self::from_f64(self.0.acosh())
    }

    fn atanh(self) -> Self {
        Self::from_f64(self.0.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}

impl fmt::Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl approx::AbsDiffEq for Scalar {
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon().into()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon.0)
    }
}
