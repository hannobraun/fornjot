use fj_math::Point;

use crate::{
    objects::{Objects, Shell},
    services::Service,
};

use super::FaceBuilder;

/// Builder API for [`Shell`]
pub struct ShellBuilder {}

impl ShellBuilder {
    /// Create a tetrahedron from the provided points
    pub fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        objects: &mut Service<Objects>,
    ) -> Shell {
        let [a, b, c, d] = points.map(Into::into);

        let (bottom, [ab, bc, ca]) =
            FaceBuilder::triangle([a, b, c], [None, None, None], objects);
        let (front, [_, bd, da]) =
            FaceBuilder::triangle([a, b, d], [Some(ab), None, None], objects);
        let (left, [_, _, dc]) = FaceBuilder::triangle(
            [c, a, d],
            [Some(ca), Some(da), None],
            objects,
        );
        let (back_right, _) = FaceBuilder::triangle(
            [b, c, d],
            [Some(bc), Some(dc), Some(bd)],
            objects,
        );

        Shell::new([bottom, front, left, back_right])
    }
}
