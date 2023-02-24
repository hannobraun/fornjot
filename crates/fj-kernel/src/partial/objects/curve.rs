use fj_math::Scalar;

use crate::geometry::path::Curve;

/// A possibly undefined curve
#[derive(Clone, Copy, Debug)]
pub enum MaybeCurve {
    /// The curve is fully defined
    Defined(Curve),

    /// The curve is undefined, but we know it is a circle
    UndefinedCircle {
        /// The radius of the undefined circle
        radius: Scalar,
    },

    /// The curve is undefined, but we know it is a line
    UndefinedLine,
}

impl From<Curve> for MaybeCurve {
    fn from(path: Curve) -> Self {
        Self::Defined(path)
    }
}
