use fj_math::Scalar;

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{CurveKind, Face, Shell, Surface},
};

use super::{Path, Sweep};

impl Sweep for Face {
    type Swept = Shell;

    fn sweep(self, path: impl Into<Path>) -> Self::Swept {
        let path = path.into();

        let mut faces = Vec::new();

        let is_negative_sweep = {
            let Surface::SweptCurve(surface) = self.surface();

            let a = match surface.curve {
                CurveKind::Circle(_) => todo!(
                    "Sweeping from faces defined in round surfaces is not \
                    supported"
                ),
                CurveKind::Line(line) => line.direction(),
            };
            let b = surface.path;

            let normal = a.cross(&b);

            normal.dot(&path.inner()) < Scalar::ZERO
        };

        let bottom_face = create_bottom_face(&self, is_negative_sweep);
        faces.push(bottom_face);

        let top_face = create_top_face(self.clone(), path, is_negative_sweep);
        faces.push(top_face);

        for cycle in self.all_cycles() {
            for &edge in cycle.edges() {
                let edge = if is_negative_sweep {
                    edge.reverse_including_curve()
                } else {
                    edge
                };
                let face = (edge, self.color()).sweep(path);
                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

fn create_bottom_face(face: &Face, is_negative_sweep: bool) -> Face {
    if is_negative_sweep {
        face.clone()
    } else {
        face.clone().reverse()
    }
}

fn create_top_face(face: Face, path: Path, is_negative_sweep: bool) -> Face {
    let mut face = face.translate(path.inner());

    if is_negative_sweep {
        face = face.reverse();
    };

    face
}
