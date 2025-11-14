use std::{
    cmp,
    f64::consts::{PI, TAU},
    fmt,
    hash::{Hash, Hasher},
    num::FpCategory,
    ops,
};

use decorum::{R64, divergence::OrPanic};

/// # A rational, finite scalar value
///
/// `Scalar` is a wrapper around `f64` which guarantees that the contained value
/// is both rational and finite. This allows `Scalar` to provide
/// implementations of [`Eq`], [`Ord`], and [`Hash`]; enabling `Scalar` (and
/// types built on top of it), to be used as keys in various sets and maps.
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    /// # The `Scalar` instance that represents zero
    pub const ZERO: Self = Self { value: 0. };

    /// # The `Scalar` instance that represents one
    pub const ONE: Self = Self { value: 1. };

    /// # The `Scalar` instance that represents two
    pub const TWO: Self = Self { value: 2. };

    /// # The smallest `Scalar` value
    pub const MIN: Self = Self { value: f64::MIN };

    /// # The largest `Scalar` value
    pub const MAX: Self = Self { value: f64::MAX };

    /// # The `Scalar` instance that represents π (pi)
    pub const PI: Self = Self { value: PI };

    /// # The `Scalar` instance that represents τ (tau)
    pub const TAU: Self = Self { value: TAU };

    /// # Construct a `Scalar` from an `f64`
    ///
    /// ## Panics
    ///
    /// Panics, if the value provided is `NaN` or infinite.
    pub fn from_f64(value: f64) -> Self {
        if value.is_nan() {
            panic!("`Scalar` value must not be `NaN`");
        }
        if value.is_infinite() {
            panic!("`Scalar` value must not be infinite. Value: `{value}`");
        }

        Self { value }
    }

    /// # Convert the scalar value into an `f32`
    pub fn into_f32(self) -> f32 {
        self.value as f32
    }

    /// # Convert the scalar value into an `f64`
    pub fn into_f64(self) -> f64 {
        self.value
    }

    /// # Convert the scalar value into a `u64`
    pub fn into_u64(self) -> u64 {
        self.value as u64
    }

    /// # Indicate whether the scalar value is negative
    pub fn is_negative(self) -> bool {
        self < Self::ZERO
    }

    /// # Indicate whether the scalar value is positive
    pub fn is_positive(self) -> bool {
        self > Self::ZERO
    }

    /// # Indicate whether the scalar value is zero
    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// # Compute the sign of the scalar
    pub fn sign(self) -> Sign {
        if self.is_negative() {
            return Sign::Negative;
        }
        if self.is_positive() {
            return Sign::Positive;
        }
        if self.is_zero() {
            return Sign::Zero;
        }

        unreachable!("Sign is neither negative, nor positive, nor zero.")
    }

    /// # Compute the absolute scalar value
    pub fn abs(self) -> Self {
        self.value.abs().into()
    }

    /// # Compute the maximum of this and another scalar value
    pub fn max(self, other: impl Into<Self>) -> Self {
        self.value.max(other.into().value).into()
    }

    /// # Compute the largest integer smaller than or equal to the scalar value
    pub fn floor(self) -> Self {
        self.value.floor().into()
    }

    /// # Compute the smallest integer larger than or equal to the scalar value
    pub fn ceil(self) -> Self {
        self.value.ceil().into()
    }

    /// # Round the scalar value
    pub fn round(self) -> Self {
        self.value.round().into()
    }

    /// # Compute the square root of the scalar value
    pub fn sqrt(self) -> Self {
        self.value.sqrt().into()
    }

    /// # Compute the sine of the scalar value
    pub fn sin(self) -> Self {
        self.value.sin().into()
    }

    /// # Compute the cosine of the scalar value
    pub fn cos(self) -> Self {
        self.value.cos().into()
    }

    /// # Compute sine and cosine of the scalar value
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (sin.into(), cos.into())
    }

    /// # Compute the arccosine of the scalar value
    pub fn acos(self) -> Self {
        self.value.acos().into()
    }

    /// # Compute the four-quadrant arctangent of the scalar value
    pub fn atan2(self, other: Self) -> Self {
        self.value.atan2(other.value).into()
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        // Using `R64` here to make sure that this matches the `Eq`/`PartialEq`
        // implementation, as required by `Hash`.
        R64::<OrPanic>::new(self.value).eq(&R64::<OrPanic>::new(other.value))
    }
}

impl Eq for Scalar {}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let Some(ordering) = self.value.partial_cmp(&other.value) else {
            unreachable!(
                "`Scalar` is not valid, but this has been checked by the \
                constructor."
            );
        };

        ordering
    }
}

impl Hash for Scalar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The `Eq`/`PartialEq` implementation is also using `R64`. So these
        // implementations match, as required by `Hash`.
        R64::<OrPanic>::new(self.value).hash(state);
    }
}

impl From<f32> for Scalar {
    fn from(value: f32) -> Self {
        Self::from_f64(value.into())
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self::from_f64(value)
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
        self.value.neg().into()
    }
}

impl<T> ops::Add<T> for Scalar
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.value.add(rhs.into().value).into()
    }
}

impl<T> ops::Sub<T> for Scalar
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.value.sub(rhs.into().value).into()
    }
}

impl<T> ops::Mul<T> for Scalar
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.value.mul(rhs.into().value).into()
    }
}

impl<T> ops::Div<T> for Scalar
where
    T: Into<Self>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.value.div(rhs.into().value).into()
    }
}

impl<T> ops::Rem<T> for Scalar
where
    T: Into<Self>,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        self.value.rem(rhs.into().value).into()
    }
}

impl<T> ops::AddAssign<T> for Scalar
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        self.value.add_assign(rhs.into().value);
        *self = self.value.into();
    }
}

impl<T> ops::SubAssign<T> for Scalar
where
    T: Into<Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.value.sub_assign(rhs.into().value);
        *self = self.value.into();
    }
}

impl<T> ops::MulAssign<T> for Scalar
where
    T: Into<Self>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.value.mul_assign(rhs.into().value);
        *self = self.value.into();
    }
}

impl<T> ops::DivAssign<T> for Scalar
where
    T: Into<Self>,
{
    fn div_assign(&mut self, rhs: T) {
        self.value.div_assign(rhs.into().value);
        *self = self.value.into();
    }
}

impl<T> ops::RemAssign<T> for Scalar
where
    T: Into<Self>,
{
    fn rem_assign(&mut self, rhs: T) {
        self.value.rem_assign(rhs.into().value);
        *self = self.value.into();
    }
}

impl num_traits::Zero for Scalar {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
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
        self.value.abs().into()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        <f64 as num_traits::Signed>::abs_sub(&self.value, &other.value).into()
    }

    fn signum(&self) -> Self {
        <f64 as num_traits::Signed>::signum(&self.value).into()
    }

    fn is_positive(&self) -> bool {
        <f64 as num_traits::Signed>::is_positive(&self.value)
    }

    fn is_negative(&self) -> bool {
        <f64 as num_traits::Signed>::is_negative(&self.value)
    }
}

impl num_traits::ToPrimitive for Scalar {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
}

impl num_traits::Float for Scalar {
    fn nan() -> Self {
        panic!("`Scalar` can not represent `NaN`")
    }

    fn infinity() -> Self {
        panic!("`Scalar` can not represent infinity")
    }

    fn neg_infinity() -> Self {
        panic!("`Scalar` can not represent negative infinity")
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
        self.value.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    fn classify(self) -> FpCategory {
        self.value.classify()
    }

    fn floor(self) -> Self {
        Self::from_f64(self.value.floor())
    }

    fn ceil(self) -> Self {
        Self::from_f64(self.value.ceil())
    }

    fn round(self) -> Self {
        Self::from_f64(self.value.round())
    }

    fn trunc(self) -> Self {
        Self::from_f64(self.value.trunc())
    }

    fn fract(self) -> Self {
        Self::from_f64(self.value.fract())
    }

    fn abs(self) -> Self {
        Self::from_f64(self.value.abs())
    }

    fn signum(self) -> Self {
        Self::from_f64(self.value.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::from_f64(self.value.mul_add(a.value, b.value))
    }

    fn recip(self) -> Self {
        Self::from_f64(self.value.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self::from_f64(self.value.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self::from_f64(self.value.powf(n.value))
    }

    fn sqrt(self) -> Self {
        Self::from_f64(self.value.sqrt())
    }

    fn exp(self) -> Self {
        Self::from_f64(self.value.exp())
    }

    fn exp2(self) -> Self {
        Self::from_f64(self.value.exp2())
    }

    fn ln(self) -> Self {
        Self::from_f64(self.value.ln())
    }

    fn log(self, base: Self) -> Self {
        Self::from_f64(self.value.log(base.value))
    }

    fn log2(self) -> Self {
        Self::from_f64(self.value.log2())
    }

    fn log10(self) -> Self {
        Self::from_f64(self.value.log10())
    }

    fn max(self, other: Self) -> Self {
        Self::from_f64(self.value.max(other.value))
    }

    fn min(self, other: Self) -> Self {
        Self::from_f64(self.value.min(other.value))
    }

    fn abs_sub(self, other: Self) -> Self {
        (self - other).abs()
    }

    fn cbrt(self) -> Self {
        Self::from_f64(self.value.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self::from_f64(self.value.hypot(other.value))
    }

    fn sin(self) -> Self {
        Self::from_f64(self.value.sin())
    }

    fn cos(self) -> Self {
        Self::from_f64(self.value.cos())
    }

    fn tan(self) -> Self {
        Self::from_f64(self.value.tan())
    }

    fn asin(self) -> Self {
        Self::from_f64(self.value.asin())
    }

    fn acos(self) -> Self {
        Self::from_f64(self.value.acos())
    }

    fn atan(self) -> Self {
        Self::from_f64(self.value.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self::from_f64(self.value.atan2(other.value))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        (Self::from_f64(sin), Self::from_f64(cos))
    }

    fn exp_m1(self) -> Self {
        Self::from_f64(self.value.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::from_f64(self.value.ln_1p())
    }

    fn sinh(self) -> Self {
        Self::from_f64(self.value.sinh())
    }

    fn cosh(self) -> Self {
        Self::from_f64(self.value.cosh())
    }

    fn tanh(self) -> Self {
        Self::from_f64(self.value.tanh())
    }

    fn asinh(self) -> Self {
        Self::from_f64(self.value.asinh())
    }

    fn acosh(self) -> Self {
        Self::from_f64(self.value.acosh())
    }

    fn atanh(self) -> Self {
        Self::from_f64(self.value.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

impl fmt::Debug for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <f64 as fmt::Debug>::fmt(&self.value, f)
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <f64 as fmt::Display>::fmt(&self.value, f)
    }
}

impl approx::AbsDiffEq for Scalar {
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon().into()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.value.abs_diff_eq(&other.value, epsilon.value)
    }
}

/// # The sign of a [`Scalar`]
///
/// See [`Scalar::sign`]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Sign {
    /// # The scalar value is negative
    Negative,

    /// # The scalar value is positive
    Positive,

    /// # The scalar value is zero
    Zero,
}

impl Sign {
    /// # Indicate whether the sign is negative
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative)
    }

    /// # Indicate whether the sign is positive
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive)
    }

    /// # Indicate whether the sign is zero
    pub fn is_zero(&self) -> bool {
        matches!(self, Self::Zero)
    }

    /// # Convert this sign back to a scalar
    ///
    /// Returns `-Scalar::ONE`, if the sign is negative; `Scalar::ONE`, if it is
    /// positive; or `Scalar::ZERO`, if the value is zero.
    pub fn to_scalar(self) -> Scalar {
        match self {
            Self::Negative => -Scalar::ONE,
            Self::Positive => Scalar::ONE,
            Self::Zero => Scalar::ZERO,
        }
    }
}
