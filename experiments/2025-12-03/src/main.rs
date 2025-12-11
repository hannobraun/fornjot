use fj_interop::{Color, MeshTriangle, TriMesh};

use crate::{
    objects::{
        geometry::{Triangle, Triangles},
        topology::{Faces, Solid},
    },
    operations::{face, reverse, sweep},
    store::Store,
};

mod objects;
mod operations;
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
    let mut triangles = Triangles::default();

    // Topology
    let mut half_edges = Store::default();
    let mut faces = Faces::default();
    let mut solids = Store::default();

    // Push initial vertex.
    let v0 = vertices.push([0., 0., 0.]);

    // Sweep initial vertex into lower-left edge.
    let e02 = sweep::vertex_to_half_edge(
        v0,
        [0., 1., 0.],
        &mut vertices,
        &mut half_edges,
    );

    // Sweep lower-left edge into bottom face.
    let f0264 = sweep::half_edge_to_face(
        e02,
        [1., 0., 0.],
        &mut vertices,
        &mut triangles,
        &mut half_edges,
        &mut faces,
    );

    // Prepare all the bottom edges we're going to need for the side faces.
    let [e04, e46, e62, e20] = {
        let f0462 = reverse::face(
            f0264,
            &mut half_edges,
            &mut faces,
            &mut triangles,
            &vertices,
        );

        faces[f0462].boundary
    };

    // Sweep lower-left edge into left face.
    let f2013 = sweep::half_edge_to_face(
        e20,
        [0., 0., 1.],
        &mut vertices,
        &mut triangles,
        &mut half_edges,
        &mut faces,
    );

    // Complete front face from the parts we already have.
    let f1045 = {
        let [v4, _] = half_edges[e46].vertices;

        let [_, e01, _, _] = faces[f2013].boundary;
        let e10 = reverse::half_edge(e01, &mut half_edges);

        let v5 = vertices.push(vertices[v4].position + [0., 0., 1.]);

        face::from_two_half_edges_and_vertex(
            [e10, e04],
            v5,
            &vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        )
    };

    // Complete right face from the parts we already have.
    let f5467 = {
        let [v6, _] = half_edges[e62].vertices;

        let [_, _, e45, _] = faces[f1045].boundary;
        let e54 = reverse::half_edge(e45, &mut half_edges);

        let v7 = vertices.push(vertices[v6].position + [0., 0., 1.]);

        face::from_two_half_edges_and_vertex(
            [e54, e46],
            v7,
            &vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        )
    };

    // Complete back face from the parts we already have.
    let f7623 = {
        let [_, _, e67, _] = faces[f5467].boundary;
        let e76 = reverse::half_edge(e67, &mut half_edges);

        let [_, _, _, e32] = faces[f2013].boundary;
        let e23 = reverse::half_edge(e32, &mut half_edges);

        face::from_three_half_edges(
            [e76, e62, e23],
            &vertices,
            &mut triangles,
            &mut half_edges,
            &mut faces,
        )
    };

    // Complete top face from the parts we already have.
    let f1573 = {
        let [_, _, _, e51] = faces[f1045].boundary;
        let e15 = reverse::half_edge(e51, &mut half_edges);

        let [_, _, _, e75] = faces[f5467].boundary;
        let e57 = reverse::half_edge(e75, &mut half_edges);

        let [_, _, _, e37] = faces[f7623].boundary;
        let e73 = reverse::half_edge(e37, &mut half_edges);

        let [_, _, e13, _] = faces[f2013].boundary;
        let e31 = reverse::half_edge(e13, &mut half_edges);

        face::from_four_half_edges(
            [e15, e57, e73, e31],
            &vertices,
            &half_edges,
            &mut triangles,
            &mut faces,
        )
    };

    solids.push(Solid {
        boundary: [f0264, f2013, f1045, f5467, f7623, f1573],
    });

    let mut tri_mesh = TriMesh::new();

    let triangles = solids
        .into_iter()
        .flat_map(|solid: Solid| solid.boundary)
        .flat_map(|f0123| faces[f0123].triangles)
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
