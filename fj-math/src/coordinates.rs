use super::Scalar;

/// 1-dimensional curve coordinates
#[repr(C)]
pub struct T {
    /// The single coordinate of the 1-dimensional curve coordinates
    pub t: Scalar,
}

/// 2-dimensional surface coordinates
#[repr(C)]
pub struct Uv {
    /// The first coordinate of the 2-dimensional surface coordinates
    pub u: Scalar,

    /// The second coordinate of the 2-dimensional surface coordinates
    pub v: Scalar,
}

/// 3-dimensional model coordinates
#[repr(C)]
pub struct Xyz {
    /// The first coordinate of the 3-dimensional model coordinates
    pub x: Scalar,

    /// The second coordinate of the 3-dimensional model coordinates
    pub y: Scalar,

    /// The third coordinate of the 3-dimensional model coordinates
    pub z: Scalar,
}
