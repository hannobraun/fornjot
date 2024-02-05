use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            sweep::SweepSketch,
            update::UpdateSketch,
        },
    },
    math::{Scalar, Vector},
};

pub fn model(
    size: impl Into<Vector<3>>,
    core: &mut fj::core::Instance,
) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.services.objects.surfaces.xy_plane();
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
                core,
            )
            .insert(&mut core.services),
        )
        .sweep_sketch(bottom_surface, sweep_path, &mut core.services)
}
