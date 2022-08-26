use fj_interop::{debug::DebugInfo, mesh::Color};
use fj_kernel::{
    algorithms::approx::Tolerance,
    iter::ObjectIters,
    objects::{Curve, Cycle, Edge, Face, GlobalCurve, Sketch},
    validation::{validate, Validated, ValidationConfig, ValidationError},
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
                    let cycle = add_cycle(cycle.clone(), false);
                    exteriors.push(cycle);
                }
                for cycle in face.interiors() {
                    let cycle = add_cycle(cycle.clone(), true);
                    interiors.push(cycle);
                }
            }

            for face in b.face_iter() {
                assert_eq!(
                    surface,
                    face.surface(),
                    "Trying to subtract faces with different surfaces.",
                );

                for cycle in face.exteriors() {
                    let cycle = add_cycle(cycle.clone(), true);
                    interiors.push(cycle);
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

fn add_cycle(cycle: Cycle, reverse: bool) -> Cycle {
    let mut edges = Vec::new();
    for edge in cycle.edges() {
        let curve_local = if reverse {
            edge.curve().kind().reverse()
        } else {
            *edge.curve().kind()
        };

        let curve_global = GlobalCurve::from_kind(if reverse {
            edge.curve().global().kind().reverse()
        } else {
            *edge.curve().global().kind()
        });

        let vertices = if reverse {
            edge.vertices().reverse()
        } else {
            *edge.vertices()
        };

        let edge = Edge::from_curve_and_vertices(
            Curve::new(curve_local, curve_global),
            vertices,
        );

        edges.push(edge);
    }

    if reverse {
        edges.reverse();
    }

    Cycle::new(*cycle.surface()).with_edges(edges)
}
