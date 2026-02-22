use crate::{
    Core,
    geometry::{
        SurfaceGeom,
        repr::tri_mesh::TriMesh,
        surfaces::{SweptCurve, TransformedSurface},
    },
    math::Transform,
    operations::insert::Insert,
    storage::Handle,
    topology::Surface,
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
                        generator: Box::new(TransformedSurface {
                            surface: self.clone(),
                            transform: *transform,
                        }),
                        geometry: TriMesh::empty(),
                    },
                );

                surface
            })
            .clone()
    }
}
