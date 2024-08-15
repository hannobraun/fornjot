use crate::{
    geometry::Path,
    operations::{geometry::UpdateCurveGeometry, insert::Insert},
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
        path: Path<2>,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Handle<Curve> {
        Curve::new().insert(core).make_path_on_surface(
            path,
            surface,
            &mut core.layers.geometry,
        )
    }
}

impl BuildCurve for Curve {}
