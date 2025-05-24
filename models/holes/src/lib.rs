use fj::{
    core::{
        operations::{
            holes::{AddHole, HoleLocation},
            update::UpdateSolid,
        },
        topology::Solid,
    },
    math::Scalar,
};

pub fn model(radius: impl Into<Scalar>, core: &mut fj::core::Core) -> Solid {
    let radius = radius.into();

    let size = radius * 4.;
    let cuboid = cuboid::model([size * 2., size, size], core);

    cuboid.update_shell(
        cuboid.shells().only(),
        |shell, core| {
            let bottom_face = shell
                .faces()
                .nth(5)
                .expect("Expected shell to have bottom face");
            let offset = size / 2.;
            let depth = size / 2.;

            let shell = shell.add_blind_hole(
                HoleLocation {
                    face: bottom_face,
                    position: [-offset, Scalar::ZERO].into(),
                },
                radius,
                [Scalar::ZERO, Scalar::ZERO, depth],
                core,
            );

            let bottom_face = shell
                .faces()
                .nth(5)
                .expect("Expected shell to have bottom face");
            let top_face = shell.faces().first();

            [shell.add_through_hole(
                [
                    HoleLocation {
                        face: bottom_face,
                        position: [offset, Scalar::ZERO].into(),
                    },
                    HoleLocation {
                        face: top_face,
                        position: [offset, Scalar::ZERO].into(),
                    },
                ],
                radius,
                core,
            )]
        },
        core,
    )
}
