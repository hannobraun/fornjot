use fj::core::{
    objects::{Shell, Solid},
    operations::{
        build::{BuildShell, BuildSolid},
        insert::Insert,
        update::UpdateSolid,
    },
    services::Services,
    storage::Handle,
};

pub fn model(services: &mut Services) -> Handle<Solid> {
    Solid::empty()
        .add_shells([Shell::from_vertices_and_indices(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            [[2, 1, 0], [0, 1, 3], [1, 2, 3], [2, 0, 3]],
            services,
        )
        .insert(services)])
        .insert(services)
}
