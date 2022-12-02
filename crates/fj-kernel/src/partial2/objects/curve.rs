use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, GlobalCurve, Surface},
    partial2::{Partial, PartialObject},
};

/// A partial [`Curve`]
#[derive(Clone)]
pub struct PartialCurve {
    /// The path that defines the curve
    pub path: Option<SurfacePath>,

    /// The surface the curve is defined in
    pub surface: Partial<Surface>,

    /// The global form of the curve
    pub global_form: Partial<GlobalCurve>,
}

impl PartialObject for PartialCurve {
    type Full = Curve;
}

/// A partial [`GlobalCurve`]
#[derive(Clone)]
pub struct PartialGlobalCurve;

impl PartialObject for PartialGlobalCurve {
    type Full = GlobalCurve;
}
