use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Triangle, Vector};

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
    let mut vertices = Store::default();
    let mut triangles = Vec::new();

    let mut edges = Store::default();

    // Push initial vertex.
    let v0 = vertices.push([0., 0., 0.]);

    // Sweep initial vertex into bottom-front edge.
    let v4 = {
        let e0 =
            sweep_vertex_to_edge(v0, [1., 0., 0.], &mut vertices, &mut edges);

        edges[e0].vertices[1]
    };

    // Push rest of vertices in an unstructured manner.
    let v1 = vertices.push([0., 0., 1.]);
    let v2 = vertices.push([0., 1., 0.]);
    let v3 = vertices.push([0., 1., 1.]);
    let v5 = vertices.push([1., 0., 1.]);
    let v6 = vertices.push([1., 1., 0.]);
    let v7 = vertices.push([1., 1., 1.]);

    // front
    triangles.push([v0, v4, v5]); // t0
    triangles.push([v0, v5, v1]); // t1
    // right
    triangles.push([v4, v6, v7]); // t2
    triangles.push([v4, v7, v5]); // t3
    // back
    triangles.push([v6, v2, v3]); // t4
    triangles.push([v6, v3, v7]); // t5
    // left
    triangles.push([v2, v0, v1]); // t6
    triangles.push([v2, v1, v3]); // t7
    // bottom
    triangles.push([v2, v6, v4]); // t8
    triangles.push([v2, v4, v0]); // t9
    // top
    triangles.push([v1, v5, v7]); // t10
    triangles.push([v1, v7, v3]); // t11

    let mut tri_mesh = TriMesh::new();

    for [a, b, c] in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: Triangle::from_points([
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

pub struct Edge {
    pub vertices: [Index<Vertex>; 2],
}
