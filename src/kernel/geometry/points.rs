use std::ops::{Add, Deref, DerefMut, Sub};

use crate::math::{self, Vector};

/// A point that can be losslessly converted into its canonical form
///
/// The canonical form is always the 3D representation. It needs to be provided
/// when constructing the point, along with the point's native form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<const D: usize> {
    /// This point's native form
    ///
    /// The native form of the point is its representation in its native
    /// coordinate system. This could be a 1-dimensional curve, 2-dimensional
    /// surface, or 3-dimensional model coordinate system.
    native: math::Point<D>,

    /// The canonical form of the point
    ///
    /// This is always the 3D representation of the point. Since this is always
    /// kept here, unchanged, as the point is converted into other coordinate
    /// systems, it allows for a lossless conversion back into 3D coordinates,
    /// unaffected by floating point accuracy issues.
    canonical: math::Point<3>,
}

impl<const D: usize> Point<D> {
    /// Construct a new instance
    ///
    /// Both the native and the canonical form must be provide. The caller must
    /// guarantee that both of them match.
    pub fn new(native: math::Point<D>, canonical: math::Point<3>) -> Self {
        Self { native, canonical }
    }

    /// Access the point's native form
    pub fn native(&self) -> math::Point<D> {
        self.native
    }

    /// Access the point's canonical form
    pub fn canonical(&self) -> math::Point<3> {
        self.canonical
    }
}

impl<const D: usize> Deref for Point<D> {
    type Target = math::Point<D>;

    fn deref(&self) -> &Self::Target {
        &self.native
    }
}

impl<const D: usize> DerefMut for Point<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.native
    }
}

impl From<math::Point<3>> for Point<3> {
    fn from(point: math::Point<3>) -> Self {
        Self::new(point, point)
    }
}

// Some math operations for convenience. Obviously those can never return a new
// `Point`, or the conversion back to 3D would be broken.

impl<const D: usize> Add<Vector<D>> for Point<D> {
    type Output = math::Point<D>;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        self.native.add(rhs)
    }
}

impl<const D: usize> Sub<Self> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.native.sub(rhs.native)
    }
}

impl<const D: usize> Sub<math::Point<D>> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: math::Point<D>) -> Self::Output {
        self.native.sub(rhs)
    }
}
