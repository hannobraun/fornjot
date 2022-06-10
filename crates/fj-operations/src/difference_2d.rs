use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    shape::{LocalForm, Shape, ValidationError},
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

                for cycle in face.exteriors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, &mut difference, false)?;
                    exteriors.push(cycle);
                }
                for cycle in face.interiors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, &mut difference, true)?;
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

                for cycle in face.exteriors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, &mut difference, true)?;
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
    cycle: LocalForm<Cycle<2>, Cycle<3>>,
    shape: &mut Shape,
    reverse: bool,
) -> Result<LocalForm<Cycle<2>, Cycle<3>>, ValidationError> {
    let mut edges = Vec::new();
    for edge in cycle.local().edges.clone() {
        let curve_local = *edge.local().curve.local();
        let curve_local = if reverse {
            curve_local.reverse()
        } else {
            curve_local
        };

        let curve_canonical = edge.canonical().get().curve();
        let curve_canonical = if reverse {
            curve_canonical.reverse()
        } else {
            curve_canonical
        };
        let curve_canonical = shape.insert(curve_canonical)?;

        let vertices = if reverse {
            edge.local().vertices.clone().reverse()
        } else {
            edge.local().vertices.clone()
        };

        let edge_local = Edge {
            curve: LocalForm::new(curve_local, curve_canonical.clone()),
            vertices: vertices.clone(),
        };
        let edge_canonical =
            shape.merge(Edge::new(curve_canonical, vertices))?;

        edges.push(LocalForm::new(edge_local, edge_canonical));
    }

    if reverse {
        edges.reverse();
    }

    let cycle_local = Cycle {
        edges: edges.clone(),
    };
    let cycle_canonical = shape
        .insert(Cycle::new(edges.into_iter().map(|edge| edge.canonical())))?;

    Ok(LocalForm::new(cycle_local, cycle_canonical))
}
