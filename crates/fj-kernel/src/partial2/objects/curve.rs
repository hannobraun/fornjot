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

impl PartialCurve {
    /// Construct an instance of `PartialCurve`
    pub fn new(
        path: Option<SurfacePath>,
        surface: Option<Partial<Surface>>,
        global_form: Option<Partial<GlobalCurve>>,
    ) -> Self {
        let surface = surface.unwrap_or_default();
        let global_form = global_form.unwrap_or_default();

        Self {
            path,
            surface,
            global_form,
        }
    }
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

impl PartialGlobalCurve {
    /// Construct an instance of `PartialGlobalCurve`
    pub fn new() -> Self {
        Self
    }
}

impl PartialObject for PartialGlobalCurve {
    type Full = GlobalCurve;

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        GlobalCurve
    }
}
