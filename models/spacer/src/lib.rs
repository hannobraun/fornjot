use fj::{
    core::{
        algorithms::sweep::Sweep,
        objects::{Cycle, Region, Sketch, Solid},
        operations::{
            BuildCycle, BuildRegion, BuildSketch, Insert, Reverse,
            UpdateRegion, UpdateSketch,
        },
        services::Services,
        storage::Handle,
    },
    math::{Point, Vector},
};

pub fn model(outer: f64, inner: f64, height: f64) -> Handle<Solid> {
    let mut services = Services::new();

    let sketch = Sketch::empty()
        .add_region(
            Region::circle(Point::origin(), outer, &mut services)
                .add_interiors([Cycle::circle(
                    Point::origin(),
                    inner,
                    &mut services,
                )
                .reverse(&mut services)
                .insert(&mut services)])
                .insert(&mut services),
        )
        .insert(&mut services);

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., height]);
    (sketch, surface).sweep(path, &mut services)
}
