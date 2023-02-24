use fj_math::Scalar;

use crate::geometry::path::SurfacePath;

/// The definition of a surface path within a partial object
///
/// Can be a fully defined [`SurfacePath`], or just the type of path might be
/// known.
#[derive(Clone, Copy, Debug)]
pub enum MaybeSurfacePath {
    /// The surface path is fully defined
    Defined(SurfacePath),

    /// The surface path is undefined, but we know it is a circle
    UndefinedCircle {
        /// The radius of the undefined circle
        radius: Scalar,
    },

    /// The surface path is undefined, but we know it is a line
    UndefinedLine,
}

impl From<SurfacePath> for MaybeSurfacePath {
    fn from(path: SurfacePath) -> Self {
        Self::Defined(path)
    }
}
