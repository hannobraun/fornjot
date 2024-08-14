use fj::{
    core::{
        operations::{
            build::{BuildCycle, BuildRegion, BuildSketch},
            reverse::Reverse,
            sweep::SweepSketch,
            update::{UpdateRegion, UpdateSketch},
        },
        topology::{Cycle, Region, Sketch, Solid},
    },
    math::{Point, Vector},
};

pub fn model(
    outer: f64,
    inner: f64,
    height: f64,
    core: &mut fj::core::Core,
) -> Solid {
    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([0., 0., -height]);

    Sketch::empty(&core.layers.topology)
        .add_regions(
            [Region::circle(
                Point::origin(),
                outer,
                core.layers.topology.surfaces.space_2d(),
                core,
            )
            .add_interiors(
                [Cycle::circle(
                    Point::origin(),
                    inner,
                    core.layers.topology.surfaces.space_2d(),
                    core,
                )
                .reverse(core)],
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}
