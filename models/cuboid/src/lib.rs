use fj_kernel::{
    algorithms::sweep::Sweep,
    geometry::region::Region,
    objects::{Cycle, HalfEdge, Sketch, Solid},
    operations::{BuildCycle, BuildHalfEdge, Insert, UpdateCycle},
    services::Services,
    storage::Handle,
};
use fj_math::{Point, Vector};
use itertools::Itertools;

pub fn cuboid(x: f64, y: f64, z: f64) -> Handle<Solid> {
    let mut services = Services::new();

    let sketch = {
        let exterior = {
            #[rustfmt::skip]
            let rectangle = [
                [-x / 2., -y / 2.],
                [ x / 2., -y / 2.],
                [ x / 2.,  y / 2.],
                [-x / 2.,  y / 2.],
            ];

            let mut cycle = Cycle::empty();

            let segments = rectangle
                .into_iter()
                .map(Point::from)
                .circular_tuple_windows();

            for (start, end) in segments {
                let half_edge =
                    HalfEdge::line_segment([start, end], None, &mut services)
                        .insert(&mut services);

                cycle = cycle.add_half_edges([half_edge]);
            }

            cycle.insert(&mut services)
        };

        let region = Region::new(exterior, Vec::new(), None);

        Sketch::new([region]).insert(&mut services)
    };

    let surface = services.objects.surfaces.xy_plane();

    let path = Vector::from([0., 0., z]);
    (sketch, surface).sweep(path, &mut services)
}
