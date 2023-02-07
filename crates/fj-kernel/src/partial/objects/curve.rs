use fj_math::Scalar;

use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, GlobalCurve, Objects, Surface},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Curve`]
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    /// The path that defines the curve
    pub path: Option<MaybeSurfacePath>,

    /// The surface the curve is defined in
    pub surface: Partial<Surface>,

    /// The global form of the curve
    pub global_form: Partial<GlobalCurve>,
}

impl PartialObject for PartialCurve {
    type Full = Curve;

    fn from_full(curve: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            path: Some(curve.path().into()),
            surface: Partial::from_full(curve.surface().clone(), cache),
            global_form: Partial::from_full(curve.global_form().clone(), cache),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let path = match self.path.expect("Need path to build curve") {
            MaybeSurfacePath::Defined(path) => path,
            undefined => {
                panic!(
                    "Trying to build curve with undefined path: {undefined:?}"
                )
            }
        };
        let surface = self.surface.build(objects);
        let global_form = self.global_form.build(objects);

        Curve::new(surface, path, global_form)
    }
}

/// The definition of a surface path within [`PartialCurve`]
///
/// Can be a fully defined [`SurfacePath`], or just the type of path might be
/// known.
#[derive(Clone, Debug)]
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

/// A partial [`GlobalCurve`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalCurve;

impl PartialObject for PartialGlobalCurve {
    type Full = GlobalCurve;

    fn from_full(_: &Self::Full, _: &mut FullToPartialCache) -> Self {
        Self
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        GlobalCurve
    }
}
