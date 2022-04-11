use std::collections::HashMap;

use fj_interop::DebugInfo;
use fj_kernel::{
    shape::{Handle, Shape},
    topology::{Cycle, Edge, Face, Vertex},
};
use fj_math::{Aabb, Scalar};

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut shape = Shape::new();

        // Can be cleaned up, once `each_ref` is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        let [a, b] = self.shapes();
        let shapes = [&a, &b];
        let [mut a, mut b] =
            shapes.map(|shape| shape.to_shape(tolerance, debug_info));

        // Check preconditions.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/95
        for shape in [&mut a, &mut b] {
            if shape.topology().cycles().count() != 1 {
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    cycle in each operand."
                );
            }
            if shape.topology().faces().count() != 1 {
                todo!(
                    "The 2-dimensional difference operation only supports one \
                    face in each operand."
                );
            }
        }

        // Can't panic, as we just verified that both shapes have one cycle.
        let [cycle_a, cycle_b] = [&mut a, &mut b]
            .map(|shape| shape.topology().cycles().next().unwrap());

        let mut vertices = HashMap::new();
        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        exteriors.push(add_cycle(cycle_a, &mut vertices, &mut shape));
        interiors.push(add_cycle(cycle_b, &mut vertices, &mut shape));

        // Can't panic, as we just verified that both shapes have one face.
        let [face_a, face_b] = [&mut a, &mut b]
            .map(|shape| shape.topology().faces().values().next().unwrap());

        assert!(
            face_a.surface() == face_b.surface(),
            "Trying to subtract sketches with different surfaces."
        );
        let surface = shape.insert(face_a.surface()).unwrap();

        shape
            .insert(Face::Face {
                surface,
                exteriors,
                interiors,
                color: self.color(),
            })
            .unwrap();

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.shapes()[0].bounding_volume()
    }
}

fn add_cycle(
    cycle: Handle<Cycle>,
    vertices: &mut HashMap<Vertex, Handle<Vertex>>,
    shape: &mut Shape,
) -> Handle<Cycle> {
    let mut edges = Vec::new();
    for edge in cycle.get().edges() {
        let curve = shape.insert(edge.curve()).unwrap();

        let vertices = edge.vertices().clone().map(|vs| {
            vs.map(|vertex| {
                vertices
                    .entry(vertex.clone())
                    .or_insert_with(|| {
                        let point = shape.insert(vertex.point()).unwrap();
                        shape.insert(Vertex { point }).unwrap()
                    })
                    .clone()
            })
        });

        let edge = shape.insert(Edge { curve, vertices }).unwrap();
        edges.push(edge);
    }

    shape.insert(Cycle { edges }).unwrap()
}
