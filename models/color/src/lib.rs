use fj::{
    core::{
        objects::{Region, Solid},
        operations::{
            insert::Insert,
            update::{UpdateFace, UpdateShell, UpdateSolid},
        },
        services::Services,
        storage::Handle,
    },
    interop::Color,
};

pub fn model(services: &mut Services) -> Handle<Solid> {
    let size = 1.;
    let cuboid = cuboid::model([size, size, size], services);

    cuboid
        .update_shell(cuboid.shells().only(), |shell| {
            shell
                .update_face(shell.faces().first(), |face| {
                    face.update_region(|region| {
                        Region::new(
                            region.exterior().clone(),
                            region.interiors().into_iter().cloned(),
                            Some(Color::from([0., 1., 0.])),
                        )
                        .insert(services)
                    })
                    .insert(services)
                })
                .insert(services)
        })
        .insert(services)
}
