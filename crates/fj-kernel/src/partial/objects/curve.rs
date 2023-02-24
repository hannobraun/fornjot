use fj_math::Scalar;

use crate::geometry::path::Curve;

/// The definition of a surface path within a partial object
///
/// Can be a fully defined [`Curve`], or just the type of path might be known.
#[derive(Clone, Copy, Debug)]
pub enum MaybeSurfacePath {
    /// The surface path is fully defined
    Defined(Curve),

    /// The surface path is undefined, but we know it is a circle
    UndefinedCircle {
        /// The radius of the undefined circle
        radius: Scalar,
    },

    /// The surface path is undefined, but we know it is a line
    UndefinedLine,
}

impl From<Curve> for MaybeSurfacePath {
    fn from(path: Curve) -> Self {
        Self::Defined(path)
    }
}
