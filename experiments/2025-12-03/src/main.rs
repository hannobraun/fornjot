use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Vector};

use crate::store::{Index, Store};

mod store;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        let tri_mesh = model();
        viewer.open_window().display_mesh(tri_mesh.clone());
        tri_mesh
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model() -> TriMesh {
    // Geometry
    let mut vertices = Store::default();
    let mut triangles = Store::default();

    // Topology
    let mut edges = Store::default();
    let mut faces = Store::default();

    // Push initial vertex.
    let v0 = vertices.push([0., 0., 0.]);

    // Sweep initial vertex into lower-back edge.
    let (e0, v4) = {
        let e0 =
            sweep_vertex_to_edge(v0, [1., 0., 0.], &mut vertices, &mut edges);

        (e0, edges[e0].vertices[1])
    };

    // Sweep edge into bottom face.
    let [v2, v6] = {
        let f0 = sweep_edge_to_face(
            e0,
            [0., 1., 0.],
            &mut vertices,
            &mut triangles,
            &mut edges,
            &mut faces,
        );

        let [_, e1, _, e3] = faces[f0].boundary;

        [e3, e1].map(|edge| edges[edge].vertices[1])
    };

    // Sweep edge into front face.
    let [v1, v5] = {
        let f1 = sweep_edge_to_face(
            e0,
            [0., 0., 1.],
            &mut vertices,
            &mut triangles,
            &mut edges,
            &mut faces,
        );

        let [_, e1, _, e3] = faces[f1].boundary;

        [e3, e1].map(|edge| edges[edge].vertices[1])
    };

    // Push rest of vertices in an unstructured manner.
    let v3 = vertices.push([0., 1., 1.]);
    let v7 = vertices.push([1., 1., 1.]);

    // Push rest of triangles in an unstructured manner.
    // right
    triangles.push([v4, v6, v7]); // t2
    triangles.push([v4, v7, v5]); // t3
    // back
    triangles.push([v6, v2, v3]); // t4
    triangles.push([v6, v3, v7]); // t5
    // left
    triangles.push([v2, v0, v1]); // t6
    triangles.push([v2, v1, v3]); // t7
    // top
    triangles.push([v1, v5, v7]); // t10
    triangles.push([v1, v7, v3]); // t11

    let mut tri_mesh = TriMesh::new();

    for Triangle {
        vertices: [a, b, c],
    } in triangles
    {
        tri_mesh.triangles.push(MeshTriangle {
            inner: fj_math::Triangle::from_points([
                vertices[a].position,
                vertices[b].position,
                vertices[c].position,
            ]),
            is_internal: false,
            color: Color::default(),
        });
    }

    tri_mesh
}

pub fn sweep_vertex_to_edge(
    a: Index<Vertex>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    edges: &mut Store<Edge>,
) -> Index<Edge> {
    let b = vertices.push(vertices[a].position + path.into());
    edges.push(Edge { vertices: [a, b] })
}

pub fn sweep_edge_to_face(
    e0: Index<Edge>,
    path: impl Into<Vector<3>>,
    vertices: &mut Store<Vertex>,
    triangles: &mut Store<Triangle>,
    edges: &mut Store<Edge>,
    faces: &mut Store<Face>,
) -> Index<Face> {
    let path = path.into();

    let [v0, v1] = edges[e0].vertices;

    let [e3, e1] = [v0, v1]
        .map(|vertex| sweep_vertex_to_edge(vertex, path, vertices, edges));
    let [v3, v2] = [e3, e1].map(|edge| edges[edge].vertices[1]);

    let e2 = edges.push(Edge { vertices: [v2, v3] });
    let _ = e2;

    triangles.push([v0, v1, v2]);
    triangles.push([v0, v2, v3]);

    faces.push(Face {
        boundary: [e0, e1, e2, e3],
    })
}

#[derive(Debug, Eq, PartialEq)]
pub struct Vertex {
    pub position: Point<3>,
}

impl From<[f64; 3]> for Vertex {
    fn from(position: [f64; 3]) -> Self {
        let position = position.into();
        Self { position }
    }
}

impl From<Point<3>> for Vertex {
    fn from(position: Point<3>) -> Self {
        Self { position }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Triangle {
    pub vertices: [Index<Vertex>; 3],
}

impl From<[Index<Vertex>; 3]> for Triangle {
    fn from(vertices: [Index<Vertex>; 3]) -> Self {
        Self { vertices }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    pub vertices: [Index<Vertex>; 2],
}

#[derive(Debug, Eq, PartialEq)]
pub struct Face {
    pub boundary: [Index<Edge>; 4],
}
