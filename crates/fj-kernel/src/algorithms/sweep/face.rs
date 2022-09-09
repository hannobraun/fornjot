use fj_math::{Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{CurveKind, Face, Shell, Surface},
};

use super::Sweep;

impl Sweep for Face {
    type Swept = Shell;

    fn sweep(self, path: impl Into<Vector<3>>) -> Self::Swept {
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

            normal.dot(&path) < Scalar::ZERO
        };

        let bottom_face = create_bottom_face(self.clone(), is_negative_sweep);
        faces.push(bottom_face);

        let top_face = create_top_face(self.clone(), path, is_negative_sweep);
        faces.push(top_face);

        for cycle in self.all_cycles() {
            for &half_edge in cycle.half_edges() {
                let edge = if is_negative_sweep {
                    half_edge.reverse()
                } else {
                    half_edge
                };
                let face = (edge, self.color()).sweep(path);
                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

fn create_bottom_face(face: Face, is_negative_sweep: bool) -> Face {
    if is_negative_sweep {
        face
    } else {
        face.reverse()
    }
}

fn create_top_face(
    face: Face,
    path: Vector<3>,
    is_negative_sweep: bool,
) -> Face {
    let mut face = face.translate(path);

    if is_negative_sweep {
        face = face.reverse();
    };

    face
}
