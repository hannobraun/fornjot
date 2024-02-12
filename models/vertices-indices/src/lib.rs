use fj::core::{
    objects::{Shell, Solid},
    operations::{
        build::{BuildShell, BuildSolid},
        insert::Insert,
        update::UpdateSolid,
    },
};

pub fn model(core: &mut fj::core::Instance) -> Solid {
    Solid::empty().add_shells(
        [Shell::from_vertices_and_indices(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            [[2, 1, 0], [0, 1, 3], [1, 2, 3], [2, 0, 3]],
            core,
        )
        .insert(&mut core.services)],
        core,
    )
}
