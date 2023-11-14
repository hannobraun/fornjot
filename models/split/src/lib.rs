use fj::{
    core::{
        algorithms::sweep::Sweep,
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            split::SplitEdge,
            update::{UpdateSketch, UpdateSolid},
        },
        services::Services,
        storage::Handle,
    },
    math::Vector,
};

pub fn model(
    size: f64,
    split_pos: f64,
    services: &mut Services,
) -> Handle<Solid> {
    let sketch = Sketch::empty()
        .add_region(
            Region::polygon(
                [
                    [-size / 2., -size / 2.],
                    [size / 2., -size / 2.],
                    [size / 2., size / 2.],
                    [-size / 2., size / 2.],
                ],
                services,
            )
            .insert(services),
        )
        .insert(services);

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., size]);
    let solid = (sketch, surface).sweep(path, services);

    solid
        .update_shell(solid.shells().only(), |shell| {
            shell
                .split_edge(
                    shell
                        .faces()
                        .first()
                        .region()
                        .exterior()
                        .half_edges()
                        .first(),
                    [split_pos],
                    services,
                )
                .0
                .insert(services)
        })
        .insert(services)
}
