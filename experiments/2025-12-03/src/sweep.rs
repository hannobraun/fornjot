use fj_math::Vector;

use crate::{
    geometry::{Triangle, Vertex},
    store::{Index, Store},
    topology::{Face, HalfEdge},
};

#[derive(Default)]
pub struct Sweep {}

impl Sweep {
    pub fn vertex_to_half_edge(
        &mut self,
        v0: Index<Vertex>,
        path: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Index<HalfEdge> {
        let v1 = vertices.push(vertices[v0].position + path.into());
        half_edges.push(HalfEdge { vertices: [v0, v1] })
    }

    pub fn half_edge_to_face(
        &mut self,
        e0: Index<HalfEdge>,
        path: impl Into<Vector<3>>,
        vertices: &mut Store<Vertex>,
        triangles: &mut Store<Triangle>,
        half_edges: &mut Store<HalfEdge>,
        faces: &mut Store<Face>,
    ) -> Index<Face> {
        let path = path.into();

        let [v0, v1] = half_edges[e0].vertices;

        let [e3, e1] = [v0, v1].map(|vertex| {
            self.vertex_to_half_edge(vertex, path, vertices, half_edges)
        });
        let [v3, v2] = [e3, e1].map(|edge| half_edges[edge].vertices[1]);

        let e2 = half_edges.push(HalfEdge { vertices: [v2, v3] });
        let _ = e2;

        triangles.push([v0, v1, v2]);
        triangles.push([v0, v2, v3]);

        faces.push(Face {
            boundary: [e0, e1, e2, e3],
        })
    }
}
