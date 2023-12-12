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
    math::Vector,
};

pub fn model([x, y, z]: [f64; 3], services: &mut Services) -> Handle<Solid> {
    let sketch = Sketch::empty().add_region(
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
    );

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., z]);
    sketch
        .sweep_sketch(surface, path, services)
        .insert(services)
}
