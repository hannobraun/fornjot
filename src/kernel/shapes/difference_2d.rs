use crate::{
    debug::DebugInfo,
    kernel::{
        shape::Shape,
        topology::{edges::Cycle, faces::Face},
    },
    math::{Aabb, Scalar},
};

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut shape = Shape::new();

        let [mut a, mut b] = [&self.a, &self.b]
            .map(|shape| shape.to_shape(tolerance, debug_info));

        for shape in [&mut a, &mut b] {
            if shape.cycles().all().count() != 1 {
                // See issue:
                // https://github.com/hannobraun/Fornjot/issues/95
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    cycle in each operand."
                );
            }
            if shape.faces().all().count() != 1 {
                // See issue:
                // https://github.com/hannobraun/Fornjot/issues/95
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    face in each operand."
                );
            }
        }

        // Can't panic, as we just verified that both shapes have one cycle.
        let cycles =
            [&mut a, &mut b].map(|shape| shape.cycles().all().next().unwrap());

        for cycle in cycles {
            let mut edges = Vec::new();
            for edge in &cycle.edges {
                let edge = shape.edges().add(edge.get().clone());
                edges.push(edge);
            }

            shape.cycles().add(Cycle { edges });
        }

        // Can't panic, as we just verified that both shapes have one face.
        let [face_a, face_b] =
            [&mut a, &mut b].map(|shape| shape.faces().all().next().unwrap());

        let (cycles_a, cycles_b, surface_a, surface_b) =
            match (face_a.get().clone(), face_b.get().clone()) {
                (
                    Face::Face {
                        cycles: a,
                        surface: surface_a,
                    },
                    Face::Face {
                        cycles: b,
                        surface: surface_b,
                    },
                ) => (a, b, surface_a, surface_b),
                _ => {
                    // None of the 2D types still use triangle representation.
                    unreachable!()
                }
            };

        assert!(
            surface_a == surface_b,
            "Trying to subtract sketches with different surfaces."
        );
        let surface = surface_a;

        let mut cycles = cycles_a;
        cycles.extend(cycles_b);

        shape.faces().add(Face::Face { cycles, surface });

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }
}
