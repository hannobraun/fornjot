use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    shape::{Handle, Shape, ValidationError, ValidationResult},
    topology::{Cycle, Edge, Face},
};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(
        &self,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Shape, ValidationError> {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut difference = Shape::new();

        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        // Can be cleaned up, once `each_ref` and `try_map` are stable:
        // - https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        // - https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let [a, b] = self.shapes();
        let [a, b] = [a, b].map(|shape| shape.to_shape(tolerance, debug_info));
        let [a, b] = [a?, b?];

        if let Some(face) = a.faces().next() {
            // If there's at least one face to subtract from, we can proceed.

            let surface = face.get().brep().surface.clone();

            for face in a.faces() {
                let face = face.get();
                let face = face.brep();

                assert_eq!(
                    surface.get(),
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors.as_handle() {
                    let cycle =
                        add_cycle(cycle.clone(), &mut difference, false)?;
                    exteriors.push(cycle);
                }
                for cycle in face.interiors.as_handle() {
                    let cycle =
                        add_cycle(cycle.clone(), &mut difference, true)?;
                    interiors.push(cycle);
                }
            }

            for face in b.faces() {
                let face = face.get();
                let face = face.brep();

                assert_eq!(
                    surface.get(),
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors.as_handle() {
                    let cycle =
                        add_cycle(cycle.clone(), &mut difference, true)?;
                    interiors.push(cycle);
                }
            }

            difference.merge(Face::new(
                surface,
                exteriors,
                interiors,
                self.color(),
            ))?;
        }

        Ok(difference)
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
) -> ValidationResult<Cycle<3>> {
    let mut edges = Vec::new();
    for edge in cycle.get().edges() {
        let curve = edge.curve();
        let curve = if reverse { curve.reverse() } else { curve };
        let curve = shape.insert(curve)?;

        let vertices = edge.vertices.clone().map(|vs| {
            let mut vs = vs.map(|vertex| vertex.canonical());

            if reverse {
                vs.reverse();
            }

            vs
        });

        let edge = shape.merge(Edge::new(curve, vertices))?;
        edges.push(edge);
    }

    if reverse {
        edges.reverse();
    }

    let cycle = shape.insert(Cycle::new(edges))?;

    Ok(cycle)
}
