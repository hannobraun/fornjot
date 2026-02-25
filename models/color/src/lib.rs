use fj::core::{
    operations::{
        presentation::SetColor, split::SplitFace, update::UpdateSolid,
    },
    topology::Solid,
};

pub fn model(core: &mut fj::core::Core) -> Solid {
    let size = 1.;
    let cuboid = cuboid::model_old([size, size, size], core);

    cuboid.update_shell(
        cuboid.shells().only(),
        |shell, core| {
            shell.faces().first().region().set_color([0., 1., 0.], core);

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

                let (shell, _) = shell.split_face(face, line, core);
                shell
            };

            [shell]
        },
        core,
    )
}
