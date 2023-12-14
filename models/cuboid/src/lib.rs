use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            sweep::SweepSketch,
            update::UpdateSketch,
        },
        services::Services,
        storage::Handle,
    },
    math::{Scalar, Vector},
};

pub fn model(
    size: impl Into<Vector<3>>,
    services: &mut Services,
) -> Handle<Solid> {
    let [x, y, z] = size.into().components;

    let surface = services.objects.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    Sketch::empty()
        .add_region(
            Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                services,
            )
            .insert(services),
        )
        .sweep_sketch(surface, sweep_path, services)
        .insert(services)
}
