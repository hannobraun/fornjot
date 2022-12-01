use crate::{
    geometry::path::SurfacePath,
    objects::{GlobalCurve, Surface},
    partial2::Partial,
};

/// A partial [`Curve`]
///
/// [`Curve`]: crate::objects::Curve
pub struct PartialCurve {
    /// The path that defines the curve
    pub path: Option<SurfacePath>,

    /// The surface the curve is defined in
    pub surface: Partial<Surface>,

    /// The global form of the curve
    pub global_form: Partial<GlobalCurve>,
}

/// A partial [`GlobalCurve`]
///
/// [`GlobalCurve`]: crate::objects::GlobalCurve
pub struct PartialGlobalCurve;
