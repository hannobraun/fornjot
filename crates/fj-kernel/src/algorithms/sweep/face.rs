use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse_face, TransformObject},
    objects::{Face, Shell},
};

use super::Sweep;

impl Sweep for Face {
    type Swept = Shell;

    fn sweep(
        self,
        path: impl Into<fj_math::Vector<3>>,
        tolerance: crate::algorithms::Tolerance,
        color: fj_interop::mesh::Color,
    ) -> Self::Swept {
        let path = path.into();

        let is_sweep_along_negative_direction =
            path.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO;

        let mut faces = Vec::new();

        create_bottom_faces(
            &self,
            is_sweep_along_negative_direction,
            &mut faces,
        );
        create_top_face(
            self.clone(),
            path,
            is_sweep_along_negative_direction,
            &mut faces,
        );

        for cycle in self.all_cycles() {
            for edge in cycle.edges() {
                let face = edge.sweep(path, tolerance, color);
                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

fn create_bottom_faces(
    face: &Face,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let face = if is_sweep_along_negative_direction {
        face.clone()
    } else {
        reverse_face(face)
    };

    target.push(face);
}

fn create_top_face(
    face: Face,
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let mut face = face.translate(path);

    if is_sweep_along_negative_direction {
        face = reverse_face(&face);
    };

    target.push(face);
}
