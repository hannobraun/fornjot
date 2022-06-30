use fj_math::Transform;

use crate::{
    objects::{
        Cycle, CyclesInFace, Edge, Face, FaceBRep, GlobalVertex, Vertex,
    },
    shape::LocalForm,
};

/// Transform a shape
pub fn transform_shape(faces: &mut Vec<Face>, transform: &Transform) {
    for face in faces {
        *face = transform_face(face, transform);
    }
}

pub fn transform_face(face: &Face, transform: &Transform) -> Face {
    match face {
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
    }
}

pub fn transform_cycles(
    cycles: &CyclesInFace,
    transform: &Transform,
) -> CyclesInFace {
    let cycles = cycles.as_local().map(|cycle| {
        let edges = cycle
            .edges
            .iter()
            .map(|edge| {
                let curve_local = *edge.local().curve.local();
                let curve_canonical =
                    edge.canonical().curve().transform(transform);

                let vertices = edge
                    .canonical()
                    .clone()
                    .vertices
                    .map(|vertex| transform_vertex(&vertex, transform));

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

        Cycle { edges }
    });

    CyclesInFace::new(cycles)
}

pub fn transform_vertex(vertex: &Vertex, transform: &Transform) -> Vertex {
    let position = transform.transform_point(&vertex.global().position());
    let global = GlobalVertex::from_position(position);

    Vertex::new(vertex.position(), global)
}
