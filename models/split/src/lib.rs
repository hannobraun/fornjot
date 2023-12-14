use fj::core::{
    objects::Solid,
    operations::{
        insert::Insert, split::SplitFace, sweep::SweepFaceOfShell,
        update::UpdateSolid,
    },
    services::Services,
    storage::Handle,
};

pub fn model(
    size: f64,
    split_pos: f64,
    services: &mut Services,
) -> Handle<Solid> {
    let solid = cuboid::model([size, size, size], services);

    solid
        .update_shell(solid.shells().only(), |shell| {
            let face = shell.faces().first();
            let cycle = face.region().exterior();

            let line = [
                (cycle.half_edges().nth(0).unwrap(), [split_pos]),
                (cycle.half_edges().nth(2).unwrap(), [split_pos]),
            ];

            let (shell, [face, _]) = shell.split_face(face, line, services);

            shell
                .sweep_face_of_shell(face, [0., 0., -size / 2.], services)
                .insert(services)
        })
        .insert(services)
}
