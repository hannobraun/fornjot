use std::ops::{Add, Deref, DerefMut, Sub};

use crate::math::{Point, Vector};

/// A point that can be losslessly converted into its canonical form
///
/// The canonical form is always the 3D representation. It needs to be provided
/// when constructing the point, along with the point's native form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfacePoint {
    /// This point's native form
    ///
    /// The native form of the point is its representation in its native
    /// coordinate system. This could be a 1-dimensional curve, 2-dimensional
    /// surface, or 3-dimensional model coordinate system.
    pub value: Point<2>,

    /// The canonical form of the point
    ///
    /// This is always the 3D representation of the point. Since this is always
    /// kept here, unchanged, as the point is converted into other coordinate
    /// systems, it allows for a lossless conversion back into 3D coordinates,
    /// unaffected by floating point accuracy issues.
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
        Vector::from(self.value.sub(rhs.value))
    }
}

impl Sub<Point<2>> for SurfacePoint {
    type Output = Vector<2>;

    fn sub(self, rhs: Point<2>) -> Self::Output {
        Vector::from(self.value.sub(rhs))
    }
}
