use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    iter::ObjectIters,
    objects::{Cycle, Edge, Face},
    shape::{LocalForm, Shape},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::Aabb;

use super::ToShape;

impl ToShape for fj::Difference2d {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError> {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut difference = Vec::new();

        let mut exteriors = Vec::new();
        let mut interiors = Vec::new();

        // Can be cleaned up, once `each_ref` and `try_map` are stable:
        // - https://doc.rust-lang.org/std/primitive.array.html#method.each_ref
        // - https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let [a, b] = self.shapes();
        let [a, b] =
            [a, b].map(|shape| shape.to_shape(config, tolerance, debug_info));
        let [a, b] = [a?, b?];

        if let Some(face) = a.face_iter().next() {
            // If there's at least one face to subtract from, we can proceed.

            let surface = face.brep().surface.clone();

            for face in a.face_iter() {
                let face = face.brep();

                assert_eq!(
                    surface.get(),
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, false);
                    exteriors.push(cycle);
                }
                for cycle in face.interiors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, true);
                    interiors.push(cycle);
                }
            }

            for face in b.face_iter() {
                let face = face.brep();

                assert_eq!(
                    surface.get(),
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors.as_local_form().cloned() {
                    let cycle = add_cycle(cycle, true);
                    interiors.push(cycle);
                }
            }

            difference.push(Face::new(
                surface,
                exteriors,
                interiors,
                self.color(),
            ));
        }

        validate(difference, config)
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
    reverse: bool,
) -> LocalForm<Cycle<2>, Cycle<3>> {
    let mut tmp = Shape::new();

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
        let curve_canonical = tmp.insert(curve_canonical);

        let vertices = if reverse {
            edge.local().vertices.clone().reverse()
        } else {
            edge.local().vertices.clone()
        };

        let edge_local = Edge {
            curve: LocalForm::new(curve_local, curve_canonical.clone()),
            vertices: vertices.clone(),
        };
        let edge_canonical = tmp.merge(Edge {
            curve: LocalForm::canonical_only(curve_canonical),
            vertices,
        });

        edges.push(LocalForm::new(edge_local, edge_canonical));
    }

    if reverse {
        edges.reverse();
    }

    let cycle_local = Cycle {
        edges: edges.clone(),
    };
    let cycle_canonical =
        tmp.insert(Cycle::new(edges.into_iter().map(|edge| edge.canonical())));

    LocalForm::new(cycle_local, cycle_canonical)
}
