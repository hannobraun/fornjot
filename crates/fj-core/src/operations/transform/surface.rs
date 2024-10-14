use std::rc::Rc;

use fj_math::Transform;

use crate::{
    geometry::{
        surfaces::{SweptCurve, TransformedSurface},
        SurfaceGeom,
    },
    operations::insert::Insert,
    storage::Handle,
    topology::Surface,
    Core,
};

use super::{TransformCache, TransformObject};

impl TransformObject for &Handle<Surface> {
    type Transformed = Handle<Surface>;

    fn transform_with_cache(
        self,
        transform: &Transform,
        core: &mut Core,
        cache: &mut TransformCache,
    ) -> Self::Transformed {
        cache
            .entry(self)
            .or_insert_with(|| {
                let surface = Surface::new().insert(core);

                let geometry = {
                    let SweptCurve { u, v } =
                        core.layers.geometry.of_surface(self);

                    SweptCurve {
                        u: u.transform(transform),
                        v: transform.transform_vector(v),
                    }
                };

                core.layers
                    .geometry
                    .define_surface(surface.clone(), geometry);

                core.layers.geometry.define_surface_2(
                    surface.clone(),
                    SurfaceGeom {
                        geometry: Rc::new(TransformedSurface {
                            surface: core
                                .layers
                                .geometry
                                .of_surface_2(self)
                                .unwrap()
                                .clone(),
                            transform: *transform,
                        }),
                    },
                );

                surface
            })
            .clone()
    }
}
