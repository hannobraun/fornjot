use fj_math::Transform;

use crate::{
    objects::{Curve, Cycle, CyclesInFace, Edge, Face, Surface, Vertex},
    shape::{LocalForm, Shape},
};

/// Transform the geometry of the shape
///
/// Since the topological types refer to geometry, and don't contain any
/// geometry themselves, this transforms the whole shape.
pub fn transform_shape(shape: &mut Shape, transform: &Transform) {
    shape
        .update()
        .update_all(|vertex: &mut Vertex| {
            vertex.point = transform.transform_point(&vertex.point)
        })
        .update_all(|curve: &mut Curve<3>| *curve = curve.transform(transform))
        .update_all(|surface: &mut Surface| {
            *surface = surface.transform(transform)
        })
        .update_all(|mut face: &mut Face| {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.deref_mut() {
                for (triangle, _) in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        });
}

pub fn transform_cycles(
    cycles: &CyclesInFace,
    transform: &Transform,
    target: &mut Shape,
) -> CyclesInFace {
    let cycles = cycles.as_local_form().map(|cycle| {
        let edges_local = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve_local = *edge.local().curve.local();
                let curve_canonical = target
                    .merge(edge.canonical().get().curve().transform(transform));

                let vertices = edge.canonical().get().vertices.map(|vertex| {
                    let point = vertex.canonical().get().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = target.merge(Vertex { point });

                    LocalForm::new(local, canonical)
                });

                let edge_local = Edge {
                    curve: LocalForm::new(curve_local, curve_canonical.clone()),
                    vertices: vertices.clone(),
                };
                let edge_canonical = target.merge(Edge {
                    curve: LocalForm::canonical_only(curve_canonical),
                    vertices,
                });

                LocalForm::new(edge_local, edge_canonical)
            })
            .collect();
        let edges_canonical = cycle
            .canonical()
            .get()
            .edges
            .iter()
            .map(|edge| {
                let edge = edge.canonical().get();

                let curve = {
                    let curve = edge.curve().transform(transform);

                    let curve = target.merge(curve);
                    LocalForm::canonical_only(curve)
                };
                let vertices = edge.vertices.map(|vertex| {
                    let point = vertex.canonical().get().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = target.merge(Vertex { point });

                    LocalForm::new(local, canonical)
                });

                let edge = target.merge(Edge { curve, vertices });
                LocalForm::canonical_only(edge)
            })
            .collect();

        let cycle_local = Cycle { edges: edges_local };

        let cycle_canonical = target.merge(Cycle {
            edges: edges_canonical,
        });

        LocalForm::new(cycle_local, cycle_canonical)
    });

    CyclesInFace::new(cycles)
}
