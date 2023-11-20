use std::ops::Deref;

use fj::{
    core::{
        objects::{Cycle, Region, Sketch, Solid},
        operations::{
            build::{BuildCycle, BuildRegion, BuildSketch},
            insert::Insert,
            reverse::Reverse,
            sweep::Sweep,
            update::{UpdateRegion, UpdateSketch},
        },
        services::Services,
        storage::Handle,
    },
    math::{Point, Vector},
};

pub fn model(
    outer: f64,
    inner: f64,
    height: f64,
    services: &mut Services,
) -> Handle<Solid> {
    let sketch = Sketch::empty()
        .add_region(
            Region::circle(Point::origin(), outer, services)
                .add_interiors([Cycle::circle(
                    Point::origin(),
                    inner,
                    services,
                )
                .reverse(services)
                .insert(services)])
                .insert(services),
        )
        .insert(services);

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., height]);
    (sketch.deref(), surface).sweep(path, services)
}
