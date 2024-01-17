use fj::core::{
    objects::Solid,
    operations::{
        insert::Insert,
        presentation::SetColor,
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
            shell
                .update_face(shell.faces().first(), |face| {
                    face.update_region(|region| {
                        region.set_color([0., 1., 0.]).insert(services)
                    })
                    .insert(services)
                })
                .insert(services)
        })
        .insert(services)
}
