use fj::core::{
    math::{Scalar, Vector},
    operations::{
        build::{BuildRegion, BuildSketch},
        sweep::SweepSketch,
        update::UpdateSketch,
    },
    topology::{Region, Sketch as SketchOld, Solid},
};

pub fn model_old(
    size: impl Into<Vector<3>>,
    core: &mut fj::core::Core,
) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, -z]);

    SketchOld::empty(&core.layers.topology)
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
