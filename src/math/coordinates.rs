use super::Scalar;

/// 1-dimensional curve coordinates
#[repr(C)]
pub struct T {
    pub t: Scalar,
}

/// 2-dimensional surface coordinates
#[repr(C)]
pub struct Uv {
    pub u: Scalar,
    pub v: Scalar,
}

/// 3-dimensional model coordinates
#[repr(C)]
pub struct Xyz {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}
