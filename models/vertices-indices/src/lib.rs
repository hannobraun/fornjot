use fj::core::{
    operations::{
        build::{BuildShell, BuildSolid},
        update::UpdateSolid,
    },
    topology::{Shell, Solid},
};

pub fn model(core: &mut fj::core::Core) -> Solid {
    Solid::empty().add_shells(
        [Shell::from_vertices_and_indices(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            [[2, 1, 0], [0, 1, 3], [1, 2, 3], [2, 0, 3]],
            core,
        )],
        core,
    )
}
