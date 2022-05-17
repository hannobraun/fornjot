use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    shape::{Handle, Shape},
    topology::{Cycle, Edge, Face},
};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Shape {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut difference = Shape::new();

        // Can be cleaned up, once `each_ref` is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        let [a, b] = self.shapes();
        let [a, b] = [a, b].map(|shape| shape.to_shape(tolerance, debug_info));
        let shapes = [&a, &b];

        // Check preconditions.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/95
        for shape in shapes {
            if shape.cycles().count() != 1 {
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    cycle in each operand."
                );
            }
            if shape.faces().count() != 1 {
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    face in each operand."
                );
            }
        }

        // Can't panic, as we just verified that both shapes have one face.
        let [face_a, face_b] =
            shapes.map(|shape| shape.faces().values().next().unwrap());

        assert!(
            face_a.surface() == face_b.surface(),
            "Trying to subtract sketches with different surfaces."
        );
        let surface = difference.insert(face_a.surface()).unwrap();

        // Can't panic, as we just verified that both shapes have one cycle.
        let [cycle_a, cycle_b] =
            shapes.map(|shape| shape.cycles().next().unwrap());

        let cycle_a = add_cycle(cycle_a, &mut difference, false);
        let cycle_b = add_cycle(cycle_b, &mut difference, true);

        let exteriors = vec![cycle_a];
        let interiors = vec![cycle_b];

        difference
            .insert(Face::new(surface, exteriors, interiors, self.color()))
            .unwrap();

        difference
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}

fn add_cycle(
    cycle: Handle<Cycle<3>>,
    shape: &mut Shape,
    reverse: bool,
) -> Handle<Cycle<3>> {
    let mut edges = Vec::new();
    for edge in cycle.get().edges() {
        let curve = edge.curve();
        let curve = if reverse { curve.reverse() } else { curve };
        let curve = shape.insert(curve).unwrap();

        let vertices = edge.vertices.clone().map(|vs| {
            let mut vs = vs.map(|vertex| vertex.canonical().clone());

            if reverse {
                vs.reverse();
            }

            vs
        });

        let edge = shape.merge(Edge::new(curve, vertices)).unwrap();
        edges.push(edge);
    }

    if reverse {
        edges.reverse();
    }

    shape.insert(Cycle::new(edges)).unwrap()
}
