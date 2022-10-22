use fj_interop::{debug::DebugInfo, ext::ArrayExt, mesh::Color};
use fj_kernel::{
    algorithms::{
        reverse::Reverse,
        validate::{Validate, Validated, ValidationConfig, ValidationError},
    },
    iter::ObjectIters,
    objects::{Face, Objects, Sketch},
};
use fj_math::Aabb;

use super::Shape;

impl Shape for fj::Difference2d {
    type Brep = Sketch;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        objects: &Objects,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut faces = Vec::new();

        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        let [a, b] = self.shapes().each_ref_ext().try_map_ext(|shape| {
            shape.compute_brep(config, objects, debug_info)
        })?;

        if let Some(face) = a.face_iter().next() {
            // If there's at least one face to subtract from, we can proceed.

            let surface = face.surface();

            for face in a.face_iter() {
                assert_eq!(
                    surface,
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                exteriors.push(face.exterior().clone());
                for cycle in face.interiors() {
                    interiors.push(cycle.clone().reverse(objects));
                }
            }

            for face in b.face_iter() {
                assert_eq!(
                    surface,
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                interiors.push(face.exterior().clone().reverse(objects));
            }

            // Faces only support one exterior, while the code here comes from
            // the time when a face could have multiple exteriors. This was only
            // a special case, i.e. faces that connected to themselves, and I
            // have my doubts that this code was ever correct in the first
            // place.
            //
            // Anyway, the following should make sure that at least any problems
            // this code causes become obvious. I don't know if this can ever
            // trigger, but better safe than sorry.
            let exterior = exteriors
                .pop()
                .expect("Can't construct face without an exterior");
            assert!(
                exteriors.is_empty(),
                "Can't construct face with multiple exteriors"
            );

            faces.push(
                Face::builder(objects)
                    .with_exterior(exterior)
                    .with_interiors(interiors)
                    .with_color(Color(self.color()))
                    .build(),
            );
        }

        let difference = Sketch::builder(objects).with_faces(faces).build();
        difference.validate_with_config(config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}
