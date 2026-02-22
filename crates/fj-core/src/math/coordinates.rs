use super::Scalar;

/// 1-dimensional curve coordinates
///
/// One-dimensional [`Point`]s and [`Vector`]s dereference to this type (via
/// [`Deref`]). This allows you to access the `t` field, even though [`Point`]
/// and [`Vector`] do not have such a field themselves.
///
/// [`Deref`]: std::ops::Deref
/// [`Point`]: crate::math::Point
/// [`Vector`]: crate::math::Vector
#[repr(C)]
pub struct T {
    /// The single coordinate of the 1-dimensional curve coordinates
    pub t: Scalar,
}

/// 2-dimensional surface coordinates
///
/// Two-dimensional [`Point`]s and [`Vector`]s dereference to this type (via
/// [`Deref`]). This allows you to access the `u`/`v` fields, even though
/// [`Point`] and [`Vector`] do not have such fields themselves.
///
/// [`Deref`]: std::ops::Deref
/// [`Point`]: crate::math::Point
/// [`Vector`]: crate::math::Vector
#[repr(C)]
pub struct Uv {
    /// The first coordinate of the 2-dimensional surface coordinates
    pub u: Scalar,

    /// The second coordinate of the 2-dimensional surface coordinates
    pub v: Scalar,
}

/// 3-dimensional model coordinates
///
/// Three-dimensional [`Point`]s and [`Vector`]s dereference to this type (via
/// [`Deref`]). This allows you to access the `x`/`y`/`z` fields, even though
/// [`Point`] and [`Vector`] do not have such fields themselves.
///
/// [`Deref`]: std::ops::Deref
/// [`Point`]: crate::math::Point
/// [`Vector`]: crate::math::Vector
#[repr(C)]
pub struct Xyz {
    /// The first coordinate of the 3-dimensional model coordinates
    pub x: Scalar,

    /// The second coordinate of the 3-dimensional model coordinates
    pub y: Scalar,

    /// The third coordinate of the 3-dimensional model coordinates
    pub z: Scalar,
}
