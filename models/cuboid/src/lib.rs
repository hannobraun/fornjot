use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            sweep::SweepSketch,
            update::UpdateSketch,
        },
    },
    math::{Scalar, Vector},
};

pub fn model(size: impl Into<Vector<3>>, core: &mut fj::core::Core) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.objects.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    Sketch::empty()
        .add_regions(
            [Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}
