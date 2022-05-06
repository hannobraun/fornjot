use fj_interop::debug::DebugInfo;
use fj_kernel::{algorithms::Tolerance, shape::Shape};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Difference3d {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        // Can be cleaned up, once `each_ref` is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        let [a, b] = self.shapes();
        let shapes_ref = [a, b];
        let shapes =
            shapes_ref.map(|shape| shape.to_shape(tolerance, debug_info));

        // Can be cleaned up, once `each_mut` is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.each_mut
        let [mut a, mut b] = shapes;
        let [a, b] = [&mut a, &mut b];

        // TASK: Implement algorithm from "Boundary Representation Modelling
        //       Techniques", section 6.1.1 (pages 127 ff.).

        // Check the faces of both shapes for intersections.
        for face_a in a.topology().faces() {
            for face_b in b.topology().faces() {
                let surface_a = face_a.get().surface();
                let surface_b = face_b.get().surface();

                // TASK: Check `surface_a` and `surface_b` for intersection. If
                //       that results in an intersection curve, continue.
                // TASK: Check intersection curve against each of `face_a` and
                //       `face_b`. If the two resulting list of intersections
                //       are not empty, continue.
                // TASK: Compare the two lists of intersections. If the
                //       resulting list of common intersections is not empty,
                //       continue.

                // TASK: Create edges from the common intersections. Also
                //       shorten edges where those common intersections
                //       intersect existing edges of the face. Create new
                //       vertices as boundaries of those new and shortened
                //       edges.
                //
                //       Do all of this in a way that doesn't create duplicate
                //       vertices. This not only goes for for the two faces
                //       we're looking at here, it also goes for other faces we
                //       haven't even compared to yet.
                //
                //       At this point, we probably need API to query for all
                //       faces that an edge is part of, so we can update
                //       everything properly?

                // TASK: Implement.
                let _ = surface_a;
                let _ = surface_b;
                todo!()
            }
        }

        // TASK: Implement.
        todo!()
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}
