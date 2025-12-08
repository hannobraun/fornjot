use fj_interop::{Color, MeshTriangle, TriMesh};

use crate::{
    geometry::{Triangle, Triangles, Vertex},
    store::{Index, Store},
    sweep::Sweep,
    topology::{Face, Faces, HalfEdge},
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
    let mut faces = Faces::default();

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
    let (f0264, [v6, v4]) = {
        let f0264 = sweep.half_edge_to_face(
            e02,
            [1., 0., 0.],
            &mut vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        );

        let [_, _, e64, _] = faces[f0264].boundary;
        (f0264, half_edges[e64].vertices)
    };

    let f0462 = reverse_face(
        f0264,
        &mut half_edges,
        &mut faces,
        &mut triangles,
        &vertices,
    );
    let [e04, _, _, e20] = faces[f0462].boundary;

    // Sweep lower-left edge into left face.
    let (f2013, [v1, v3]) = {
        let f2013 = sweep.half_edge_to_face(
            e20,
            [0., 0., 1.],
            &mut vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        );

        let [_, _, e13, _] = faces[f2013].boundary;
        (f2013, half_edges[e13].vertices)
    };

    // Complete front face from the parts we already have.
    let (f1045, v5) = {
        let [_, e01, _, _] = faces[f2013].boundary;
        let e10 = reverse_half_edge(e01, &mut half_edges);

        let e45 = sweep.vertex_to_half_edge(
            v4,
            [0., 0., 1.],
            &mut vertices,
            &mut half_edges,
        );

        let [_, v5] = half_edges[e45].vertices;
        let e51 = half_edges.push(HalfEdge { vertices: [v5, v1] });

        let t104 = triangles.push([v1, v0, v4], &vertices);
        let t145 = triangles.push([v1, v4, v5], &vertices);

        let f1045 = faces.push(Face {
            boundary: [e10, e04, e45, e51],
            triangles: [t104, t145],
        });

        (f1045, v5)
    };

    // Push rest of vertices in an unstructured manner.
    let v7 = vertices.push([1., 1., 1.]);

    // Push rest of triangles in an unstructured manner.
    // right
    let t467 = triangles.push([v4, v6, v7], &vertices);
    let t475 = triangles.push([v4, v7, v5], &vertices);
    // back
    let t623 = triangles.push([v6, v2, v3], &vertices);
    let t637 = triangles.push([v6, v3, v7], &vertices);
    // top
    let t157 = triangles.push([v1, v5, v7], &vertices);
    let t173 = triangles.push([v1, v7, v3], &vertices);

    let mut tri_mesh = TriMesh::new();

    let triangles = [f0264, f2013, f1045]
        .map(|f0123| faces[f0123].triangles)
        .into_iter()
        .flatten()
        .chain([t467, t475, t623, t637, t157, t173])
        .map(|t012| &triangles[t012]);

    for &Triangle {
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

pub fn reverse_triangle(
    t012: Index<Triangle>,
    triangles: &mut Triangles,
    vertices: &Store<Vertex>,
) -> Index<Triangle> {
    let [v0, v1, v2] = triangles[t012].vertices;
    triangles.push(
        Triangle {
            vertices: [v0, v2, v1],
        },
        vertices,
    )
}

pub fn reverse_half_edge(
    e: Index<HalfEdge>,
    half_edges: &mut Store<HalfEdge>,
) -> Index<HalfEdge> {
    let [v0, v1] = half_edges[e].vertices;
    half_edges.push(HalfEdge { vertices: [v1, v0] })
}

pub fn reverse_face(
    f0123: Index<Face>,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
    triangles: &mut Triangles,
    vertices: &Store<Vertex>,
) -> Index<Face> {
    let [e10, e21, e32, e03] = faces[f0123]
        .boundary
        .map(|e| reverse_half_edge(e, half_edges));

    let triangles = faces[f0123]
        .triangles
        .map(|t012| reverse_triangle(t012, triangles, vertices));

    faces.push(Face {
        boundary: [e03, e32, e21, e10],
        triangles,
    })
}
