use fj_math::Transform;

use crate::{
    objects::{Cycle, CyclesInFace, Edge, Face, FaceBRep, Vertex},
    shape::LocalForm,
};

/// Transform a shape
pub fn transform(faces: &[Face], transform: &Transform) -> Vec<Face> {
    let mut target = Vec::new();

    for face in faces {
        let face = match face {
            Face::Face(face) => {
                let surface = face.surface.transform(transform);

                let exteriors = transform_cycles(&face.exteriors, transform);
                let interiors = transform_cycles(&face.interiors, transform);

                let color = face.color;

                Face::Face(FaceBRep {
                    surface,
                    exteriors,
                    interiors,
                    color,
                })
            }
            Face::Triangles(triangles) => {
                let mut target = Vec::new();

                for &(triangle, color) in triangles {
                    let triangle = transform.transform_triangle(&triangle);
                    target.push((triangle, color));
                }

                Face::Triangles(target)
            }
        };
        target.push(face);
    }

    target
}

pub fn transform_cycles(
    cycles: &CyclesInFace,
    transform: &Transform,
) -> CyclesInFace {
    let cycles = cycles.as_local_form().map(|cycle| {
        let edges_local = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve_local = *edge.local().curve.local();
                let curve_canonical =
                    edge.canonical().curve().transform(transform);

                let vertices = edge.canonical().vertices.map(|vertex| {
                    let point = vertex.canonical().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = Vertex { point };

                    LocalForm::new(local, canonical)
                });

                let edge_local = Edge {
                    curve: LocalForm::new(curve_local, curve_canonical),
                    vertices: vertices.clone(),
                };
                let edge_canonical = Edge {
                    curve: LocalForm::canonical_only(curve_canonical),
                    vertices,
                };

                LocalForm::new(edge_local, edge_canonical)
            })
            .collect();
        let edges_canonical = cycle
            .canonical()
            .edges
            .iter()
            .map(|edge| {
                let edge = edge.canonical();

                let curve = {
                    let curve = edge.curve().transform(transform);
                    LocalForm::canonical_only(curve)
                };
                let vertices = edge.vertices.map(|vertex| {
                    let point = vertex.canonical().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = Vertex { point };

                    LocalForm::new(local, canonical)
                });

                let edge = Edge { curve, vertices };
                LocalForm::canonical_only(edge)
            })
            .collect();

        let cycle_local = Cycle { edges: edges_local };

        let cycle_canonical = Cycle {
            edges: edges_canonical,
        };

        LocalForm::new(cycle_local, cycle_canonical)
    });

    CyclesInFace::new(cycles)
}
