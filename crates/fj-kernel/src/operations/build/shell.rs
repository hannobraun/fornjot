use fj_math::Point;

use crate::{
    objects::{Face, Objects, Shell},
    operations::Insert,
    services::Service,
};

use super::{BuildFace, Triangle};

/// Build a [`Shell`]
pub trait BuildShell {
    /// Build a tetrahedron from the provided points
    fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        objects: &mut Service<Objects>,
    ) -> Shell {
        let [a, b, c, d] = points.map(Into::into);

        let Triangle {
            face: face_abc,
            edges: [ab, bc, ca],
        } = Face::triangle([a, b, c], [None, None, None], objects);
        let Triangle {
            face: face_abd,
            edges: [_, bd, da],
        } = Face::triangle([a, b, d], [Some(ab), None, None], objects);
        let Triangle {
            face: face_cad,
            edges: [_, _, dc],
        } = Face::triangle([c, a, d], [Some(ca), Some(da), None], objects);
        let Triangle { face: face_bcd, .. } =
            Face::triangle([b, c, d], [Some(bc), Some(dc), Some(bd)], objects);

        let faces = [face_abc, face_abd, face_cad, face_bcd]
            .map(|face| face.insert(objects));
        Shell::new(faces)
    }
}

impl BuildShell for Shell {}
