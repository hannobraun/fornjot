use fj_kernel::{
    algorithms::sweep::Sweep,
    geometry::region::Region,
    objects::{Cycle, Sketch, Solid},
    operations::{BuildCycle, Insert},
    services::Services,
    storage::Handle,
};
use fj_math::Vector;

pub fn cuboid(x: f64, y: f64, z: f64) -> Handle<Solid> {
    let mut services = Services::new();

    let sketch = {
        let exterior = Cycle::polygon(
            [
                [-x / 2., -y / 2.],
                [x / 2., -y / 2.],
                [x / 2., y / 2.],
                [-x / 2., y / 2.],
            ],
            &mut services,
        )
        .insert(&mut services);

        let region = Region::new(exterior, Vec::new(), None);

        Sketch::new([region]).insert(&mut services)
    };

    let surface = services.objects.surfaces.xy_plane();

    let path = Vector::from([0., 0., z]);
    (sketch, surface).sweep(path, &mut services)
}
