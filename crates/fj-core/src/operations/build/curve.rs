use crate::{
    geometry::{CurveGeom, SurfacePath},
    operations::insert::Insert,
    storage::Handle,
    topology::{Curve, Surface},
    Core,
};

/// Build a [`Curve`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildCurve {
    /// Build a curve from the provided path and surface
    fn from_path_and_surface(
        path: SurfacePath,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Handle<Curve> {
        let curve = Curve::new().insert(core);

        core.layers.geometry.define_curve(
            curve.clone(),
            CurveGeom::from_path_and_surface(path, surface),
        );

        curve
    }
}

impl BuildCurve for Curve {}
