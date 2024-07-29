use crate::{Scalar, Vector};

/// # An n-dimensional bivector
///
/// The dimensionality of the vector is defined by the generic `D` parameter.
///
/// ## Implementation Note
///
/// The bivector representation chosen here, two vectors whose outer product
/// forms the bivector, is not the only one, and it might not be the best one
/// for our needs.
///
/// I considered using a coordinate-based representation, as that would be
/// unique and require less memory, but since we need 3 coordinates for 3D, but
/// just 1 coordinate for 2D, this would require type shenanigans or the (at the
/// time of writing) unstable `generic_const_exprs` feature.
///
/// I've decided that two vectors is good enough, and anything else not worth
/// the trouble. But we might want to reconsider, once `generic_const_exprs` is
/// stable.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(C)]
pub struct Bivector<const D: usize> {
    /// The first of the vectors whose outer product defines this bivector
    pub a: Vector<D>,

    /// The second of the vectors whose outer product defines this bivector
    pub b: Vector<D>,
}

impl<const D: usize> Bivector<D> {
    /// Compute the magnitude of the bivector
    pub fn magnitude(&self) -> Scalar {
        self.a.angle_to(&self.b).sin().abs()
            * self.a.magnitude()
            * self.b.magnitude()
    }
}
