use fj::{
    core::{
        algorithms::sweep::Sweep,
        objects::{Sketch, Solid},
        operations::{BuildSketch, Insert, UpdateSketch},
        services::Services,
        storage::Handle,
    },
    math::Vector,
};

pub fn cuboid(x: f64, y: f64, z: f64) -> Handle<Solid> {
    let mut services = Services::new();

    let sketch = Sketch::empty()
        .add_polygon(
            [
                [-x / 2., -y / 2.],
                [x / 2., -y / 2.],
                [x / 2., y / 2.],
                [-x / 2., y / 2.],
            ],
            &mut services,
        )
        .insert(&mut services);

    let surface = services.objects.surfaces.xy_plane();

    let path = Vector::from([0., 0., z]);
    (sketch, surface).sweep(path, &mut services)
}
