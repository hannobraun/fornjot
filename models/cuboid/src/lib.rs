use fj::{
    core::{
        operations::{
            build::{BuildRegion, BuildSketch},
            sweep::SweepSketch,
            update::UpdateSketch,
        },
        topology::{Region, Sketch, Solid},
    },
    math::{Scalar, Vector},
};

pub fn model(size: impl Into<Vector<3>>, core: &mut fj::core::Core) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, -z]);

    Sketch::empty(&core.layers.topology)
        .add_regions(
            [Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                core.layers.topology.surfaces.space_2d(),
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}
