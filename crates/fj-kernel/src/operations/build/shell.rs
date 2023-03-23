use fj_math::Point;

use crate::{
    objects::{Face, Objects, Shell},
    services::Service,
};

use super::BuildFace;

/// Build a [`Shell`]
pub trait BuildShell {
    /// Build a tetrahedron from the provided points
    fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        objects: &mut Service<Objects>,
    ) -> Shell {
        let [a, b, c, d] = points.map(Into::into);

        let (base, [ab, bc, ca]) =
            Face::triangle([a, b, c], [None, None, None], objects);
        let (side_a, [_, bd, da]) =
            Face::triangle([a, b, d], [Some(ab), None, None], objects);
        let (side_b, [_, _, dc]) =
            Face::triangle([c, a, d], [Some(ca), Some(da), None], objects);
        let (side_c, _) =
            Face::triangle([b, c, d], [Some(bc), Some(dc), Some(bd)], objects);

        let faces = [base, side_a, side_b, side_c];
        Shell::new(faces)
    }
}

impl BuildShell for Shell {}
