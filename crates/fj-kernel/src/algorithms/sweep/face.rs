use crate::{
    algorithms::{reverse_face, TransformObject},
    objects::{Face, Shell},
};

use super::{Path, Sweep};

impl Sweep for Face {
    type Swept = Shell;

    fn sweep(
        self,
        path: impl Into<Path>,
        tolerance: crate::algorithms::Tolerance,
        color: fj_interop::mesh::Color,
    ) -> Self::Swept {
        let path = path.into();

        let mut faces = Vec::new();

        let bottom_face =
            create_bottom_face(&self, path.is_negative_direction());
        faces.push(bottom_face);

        let top_face = create_top_face(self.clone(), path);
        faces.push(top_face);

        for cycle in self.all_cycles() {
            for edge in cycle.edges() {
                let face = edge.sweep(path, tolerance, color);
                faces.push(face);
            }
        }

        Shell::new().with_faces(faces)
    }
}

fn create_bottom_face(
    face: &Face,
    is_sweep_along_negative_direction: bool,
) -> Face {
    if is_sweep_along_negative_direction {
        face.clone()
    } else {
        reverse_face(face)
    }
}

fn create_top_face(face: Face, path: Path) -> Face {
    let mut face = face.translate(path.inner());

    if path.is_negative_direction() {
        face = reverse_face(&face);
    };

    face
}
