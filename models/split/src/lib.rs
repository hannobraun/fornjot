use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildRegion, BuildSketch},
            insert::Insert,
            split::SplitFace,
            sweep::SweepSketch,
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
    let sketch = Sketch::empty().add_region(
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
    );

    let surface = services.objects.surfaces.xy_plane();
    let path = Vector::from([0., 0., size]);
    let solid = sketch.sweep_sketch(surface, path, services);

    solid
        .update_shell(solid.shells().only(), |shell| {
            let face = shell.faces().first();
            let cycle = face.region().exterior();

            let line = [
                (cycle.half_edges().nth(0).unwrap(), [split_pos]),
                (cycle.half_edges().nth(2).unwrap(), [split_pos]),
            ];

            let (shell, _) = shell.split_face(face, line, services);
            shell.insert(services)
        })
        .insert(services)
}
