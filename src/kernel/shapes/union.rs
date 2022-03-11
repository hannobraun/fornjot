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

impl ToShape for fj::Union {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let mut shape = Shape::new();

        let a = self.a.to_shape(tolerance, debug_info);
        let b = self.b.to_shape(tolerance, debug_info);

        // This doesn't create a true union, as it doesn't eliminate, merge, or
        // split faces.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/42
        copy_shape(a, &mut shape);
        copy_shape(b, &mut shape);

        shape
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let a = self.a.bounding_volume();
        let b = self.b.bounding_volume();

        a.merged(&b)
    }
}

fn copy_shape(mut orig: Shape, target: &mut Shape) {
    let mut points = HashMap::new();
    let mut curves = HashMap::new();
    let mut surfaces = HashMap::new();

    let mut vertices = HashMap::new();
    let mut edges = HashMap::new();
    let mut cycles = HashMap::new();

    for point_orig in orig.geometry().points() {
        let point = target.geometry().add_point(*point_orig.get());
        points.insert(point_orig, point);
    }
    for curve_orig in orig.geometry().curves() {
        let curve = target.geometry().add_curve(*curve_orig.get());
        curves.insert(curve_orig, curve);
    }
    for surface_orig in orig.geometry().surfaces() {
        let surface = target.geometry().add_surface(*surface_orig.get());
        surfaces.insert(surface_orig, surface);
    }

    for vertex_orig in orig.topology().vertices() {
        let vertex = target
            .topology()
            .add_vertex(Vertex {
                point: points[&vertex_orig.point].clone(),
            })
            .unwrap();
        vertices.insert(vertex_orig, vertex);
    }
    for edge_orig in orig.topology().edges() {
        let edge = target
            .topology()
            .add_edge(Edge {
                curve: curves[&edge_orig.curve].clone(),
                vertices: edge_orig.vertices.as_ref().map(|vs| {
                    vs.clone().map(|vertex| vertices[&vertex].clone())
                }),
            })
            .unwrap();
        edges.insert(edge_orig, edge);
    }
    for cycle_orig in orig.topology().cycles() {
        let cycle = target
            .topology()
            .add_cycle(Cycle {
                edges: cycle_orig
                    .edges
                    .iter()
                    .map(|edge| edges[edge].clone())
                    .collect(),
            })
            .unwrap();
        cycles.insert(cycle_orig, cycle);
    }

    for face_orig in orig.topology().faces() {
        match face_orig.get() {
            Face::Face {
                surface,
                cycles: cs,
            } => {
                target
                    .topology()
                    .add_face(Face::Face {
                        surface: surfaces[surface].clone(),
                        cycles: cs
                            .iter()
                            .map(|cycle| cycles[cycle].clone())
                            .collect(),
                    })
                    .unwrap();
            }
            face @ Face::Triangles(_) => {
                target.topology().add_face(face.clone()).unwrap();
            }
        }
    }
}
