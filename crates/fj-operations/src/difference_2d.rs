use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    algorithms::{
        approx::Tolerance,
        reverse::Reverse,
        validate::{validate, Validated, ValidationConfig, ValidationError},
    },
    iter::ObjectIters,
    objects::{Face, Sketch},
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Difference2d {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut faces = Vec::new();

        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        // Can be cleaned up, once `each_ref` and `try_map` are stable:
        // - https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        // - https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let [a, b] = self.shapes();
        let [a, b] = [a, b]
            .map(|shape| shape.compute_brep(config, tolerance, debug_info));
        let [a, b] = [a?, b?];

        if let Some(face) = a.face_iter().next() {
            // If there's at least one face to subtract from, we can proceed.

            let surface = face.surface();

            for face in a.face_iter() {
                assert_eq!(
                    surface,
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors() {
                    exteriors.push(cycle.clone());
                }
                for cycle in face.interiors() {
                    interiors.push(cycle.clone().reverse());
                }
            }

            for face in b.face_iter() {
                assert_eq!(
                    surface,
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors() {
                    interiors.push(cycle.clone().reverse());
                }
            }

            faces.push(
                Face::new(*surface)
                    .with_exteriors(exteriors)
                    .with_interiors(interiors)
                    .with_color(Color(self.color())),
            );
        }

        let difference = Sketch::new().with_faces(faces);
        validate(difference, config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}
