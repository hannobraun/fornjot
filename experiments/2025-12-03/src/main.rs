use fj_interop::{Color, MeshTriangle, TriMesh};

use crate::{
    geometry::{Triangle, Triangles},
    store::{Index, Store},
    sweep::Sweep,
    topology::{Face, HalfEdge},
};

mod geometry;
mod store;
mod sweep;
mod topology;

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
    let mut triangles = Triangles::default();

    // Topology
    let mut half_edges = Store::default();
    let mut faces = Store::default();

    // Operations
    let mut sweep = Sweep::default();

    // Push initial vertex.
    let v0 = vertices.push([0., 0., 0.]);

    // Sweep initial vertex into lower-left edge.
    let (e02, v2) = {
        let e02 = sweep.vertex_to_half_edge(
            v0,
            [0., 1., 0.],
            &mut vertices,
            &mut half_edges,
        );

        (e02, half_edges[e02].vertices[1])
    };

    // Sweep lower-left edge into bottom face.
    let (f0264, v4, v6) = {
        let f0264 = sweep.half_edge_to_face(
            e02,
            [1., 0., 0.],
            &mut vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        );

        let [_, e26, _, e40] = faces[f0264].boundary;

        (
            f0264,
            half_edges[e40].vertices[0],
            half_edges[e26].vertices[1],
        )
    };

    let f0462 = reverse_face(f0264, &mut half_edges, &mut faces);
    let [_, _, _, e20] = faces[f0462].boundary;

    // Sweep lower-left edge into left face.
    let [v1, v3] = {
        let f2013 = sweep.half_edge_to_face(
            e20,
            [0., 0., 1.],
            &mut vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        );

        let [_, e1, _, e3] = faces[f2013].boundary;

        [half_edges[e1].vertices[1], half_edges[e3].vertices[0]]
    };

    // Push rest of vertices in an unstructured manner.
    let v5 = vertices.push([1., 0., 1.]);
    let v7 = vertices.push([1., 1., 1.]);

    // Push rest of triangles in an unstructured manner.
    // right
    triangles.push([v4, v6, v7], &vertices); // t2
    triangles.push([v4, v7, v5], &vertices); // t3
    // back
    triangles.push([v6, v2, v3], &vertices); // t4
    triangles.push([v6, v3, v7], &vertices); // t5
    // front
    triangles.push([v0, v4, v5], &vertices); // t8
    triangles.push([v0, v5, v1], &vertices); // t9
    // top
    triangles.push([v1, v5, v7], &vertices); // t10
    triangles.push([v1, v7, v3], &vertices); // t11

    let mut tri_mesh = TriMesh::new();

    for Triangle {
        vertices: [a, b, c],
    } in triangles.into_store()
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

pub fn reverse_half_edge(
    e: Index<HalfEdge>,
    half_edges: &mut Store<HalfEdge>,
) -> Index<HalfEdge> {
    let [v0, v1] = half_edges[e].vertices;
    half_edges.push(HalfEdge { vertices: [v1, v0] })
}

pub fn reverse_face(
    f: Index<Face>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Store<Face>,
) -> Index<Face> {
    let [e10, e21, e32, e03] =
        faces[f].boundary.map(|e| reverse_half_edge(e, half_edges));

    faces.push(Face {
        boundary: [e03, e32, e21, e10],
    })
}
