use std::collections::HashMap;

use crate::{
    debug::DebugInfo,
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
            vertices::Vertex,
        },
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
            if shape.topology().cycles().count() != 1 {
                // See issue:
                // https://github.com/hannobraun/Fornjot/issues/95
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    cycle in each operand."
                );
            }
            if shape.topology().faces().count() != 1 {
                // See issue:
                // https://github.com/hannobraun/Fornjot/issues/95
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    face in each operand."
                );
            }
        }

        // Can't panic, as we just verified that both shapes have one cycle.
        let cycles_orig = [&mut a, &mut b]
            .map(|shape| shape.topology().cycles().next().unwrap());

        let mut vertices = HashMap::new();
        let mut cycles = Vec::new();

        for cycle in cycles_orig {
            let mut edges = Vec::new();
            for edge in &cycle.edges {
                let curve = shape.geometry().add_curve(edge.curve());

                let vertices = edge.vertices().clone().map(|vs| {
                    vs.map(|vertex| {
                        vertices
                            .entry(vertex.clone())
                            .or_insert_with(|| {
                                let point =
                                    shape.geometry().add_point(vertex.point());
                                shape
                                    .topology()
                                    .add_vertex(Vertex { point })
                                    .unwrap()
                            })
                            .clone()
                    })
                });

                let edge = shape
                    .topology()
                    .add_edge(Edge { curve, vertices })
                    .unwrap();
                edges.push(edge);
            }

            let cycle = shape.topology().add_cycle(Cycle { edges }).unwrap();
            cycles.push(cycle);
        }

        // Can't panic, as we just verified that both shapes have one face.
        let [face_a, face_b] = [&mut a, &mut b]
            .map(|shape| shape.topology().faces().next().unwrap());

        let surface_a = match (face_a.get().clone(), face_b.get().clone()) {
            (Face::Face { surface, .. }, Face::Face { .. }) => surface,
            _ => {
                // None of the 2D types still use triangle representation.
                unreachable!()
            }
        };

        assert!(
            face_a.surface() == face_b.surface(),
            "Trying to subtract sketches with different surfaces."
        );
        let surface = surface_a;

        shape
            .topology()
            .add_face(Face::Face { cycles, surface })
            .unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }
}
