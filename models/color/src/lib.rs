use fj::core::{
    objects::Solid,
    operations::{
        insert::Insert,
        presentation::SetColor,
        split::SplitFace,
        update::{UpdateFace, UpdateShell, UpdateSolid},
    },
    services::Services,
    storage::Handle,
};

pub fn model(services: &mut Services) -> Handle<Solid> {
    let size = 1.;
    let cuboid = cuboid::model([size, size, size], services);

    cuboid
        .update_shell(cuboid.shells().only(), |shell| {
            let shell = shell.update_face(shell.faces().first(), |face| {
                face.update_region(|region| {
                    region.set_color([0., 1., 0.]).insert(services)
                })
                .insert(services)
            });

            // Split colored face, to make sure the same color is applied to the
            // two derived faces.
            let shell = {
                let face = shell.faces().first();
                let line = {
                    let cycle = face.region().exterior();

                    [
                        (cycle.half_edges().nth(0).unwrap(), [0.2]),
                        (cycle.half_edges().nth(2).unwrap(), [0.2]),
                    ]
                };

                let (shell, _) = shell.split_face(face, line, services);
                shell
            };

            shell.insert(services)
        })
        .insert(services)
}
