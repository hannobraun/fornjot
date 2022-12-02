use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, GlobalCurve, Objects, Surface},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Curve`]
#[derive(Clone, Debug, Default)]
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

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let path = self.path.expect("Need path to build curve");
        let surface = self.surface.build(objects);
        let global_form = self.global_form.build(objects);

        Curve::new(surface, path, global_form)
    }
}

/// A partial [`GlobalCurve`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalCurve;

impl PartialObject for PartialGlobalCurve {
    type Full = GlobalCurve;

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        GlobalCurve
    }
}
