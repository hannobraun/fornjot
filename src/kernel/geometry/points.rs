use std::ops::{Add, Deref, DerefMut, Sub};

use crate::math::{Point, Vector};

/// A point on a surface
///
/// This type is used for algorithms that need to deal with 2D points in surface
/// coordinates. It can be converted back to the 3D point it originates from
/// without loss from floating point accuracy issues.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfacePoint {
    /// The surface coordinates of this point
    pub value: Point<2>,

    /// The 3D point this surface point was converted from
    ///
    /// Keeping this point around allows for the conversion back to a 3D point
    /// to be unaffected by floating point accuracy issues, which avoids a whole
    /// host of possible issues.
    pub from: Point<3>,
}

impl Deref for SurfacePoint {
    type Target = Point<2>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for SurfacePoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

// Some math operations for convenience. Obviously those can never return a new
// `SurfacePoint`, or the conversion back to 3D would be broken.

impl Add<Vector<2>> for SurfacePoint {
    type Output = Point<2>;

    fn add(self, rhs: Vector<2>) -> Self::Output {
        self.value.add(rhs)
    }
}

impl Sub<Self> for SurfacePoint {
    type Output = Vector<2>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.value.sub(rhs.value)
    }
}

impl Sub<Point<2>> for SurfacePoint {
    type Output = Vector<2>;

    fn sub(self, rhs: Point<2>) -> Self::Output {
        self.value.sub(rhs)
    }
}
