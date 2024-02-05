use fj::{
    core::{
        objects::{Cycle, Region, Sketch, Solid},
        operations::{
            build::{BuildCycle, BuildRegion, BuildSketch},
            insert::Insert,
            reverse::Reverse,
            sweep::SweepSketch,
            update::{UpdateRegion, UpdateSketch},
        },
    },
    math::{Point, Vector},
};

pub fn model(
    outer: f64,
    inner: f64,
    height: f64,
    core: &mut fj::core::Instance,
) -> Solid {
    let bottom_surface = core.services.objects.surfaces.xy_plane();
    let sweep_path = Vector::from([0., 0., height]);

    Sketch::empty()
        .add_region(
            Region::circle(Point::origin(), outer, core)
                .add_interiors([Cycle::circle(
                    Point::origin(),
                    inner,
                    &mut core.services,
                )
                .reverse(&mut core.services)
                .insert(&mut core.services)])
                .insert(&mut core.services),
        )
        .sweep_sketch(bottom_surface, sweep_path, &mut core.services)
}
