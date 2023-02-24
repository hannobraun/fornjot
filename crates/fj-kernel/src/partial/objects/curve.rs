use fj_math::Scalar;

use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, Objects},
    partial::{FullToPartialCache, PartialObject},
    services::Service,
};

/// A partial [`Curve`]
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    /// The path that defines the curve
    pub path: Option<MaybeSurfacePath>,
}

impl PartialObject for PartialCurve {
    type Full = Curve;

    fn from_full(curve: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self {
            path: Some(curve.path().into()),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let path = match self.path.expect("Need path to build curve") {
            MaybeSurfacePath::Defined(path) => path,
            undefined => {
                panic!(
                    "Trying to build curve with undefined path: {undefined:?}"
                )
            }
        };

        Curve::new(path)
    }
}

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
