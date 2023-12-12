use fj::{
    core::{
        objects::Solid,
        operations::{
            holes::{AddHole, HoleLocation},
            insert::Insert,
            update::UpdateSolid,
        },
        services::Services,
        storage::Handle,
    },
    math::Scalar,
};

pub fn model(
    radius: impl Into<Scalar>,
    services: &mut Services,
) -> Handle<Solid> {
    let radius = radius.into();

    let size = radius * 4.;
    let cuboid = cuboid::model([size, size, size], services);

    cuboid
        .update_shell(cuboid.shells().first(), |shell| {
            let bottom_face = shell.faces().first();
            let depth = size / 2.;

            shell
                .add_blind_hole(
                    HoleLocation {
                        face: bottom_face,
                        position: [0., 0.].into(),
                    },
                    radius,
                    [Scalar::ZERO, Scalar::ZERO, depth],
                    services,
                )
                .insert(services)
        })
        .insert(services)
}
