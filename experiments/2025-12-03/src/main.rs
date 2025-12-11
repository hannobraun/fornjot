use fj_interop::{Color, MeshTriangle, TriMesh};

use crate::{
    objects::{
        geometry::{Triangle, Triangles},
        topology::{Faces, Solid},
    },
    operations::sweep,
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
    let mut solids = Store::<Solid>::default();

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

    let s01234567 = sweep::face_to_solid(
        f0264,
        &mut vertices,
        &mut triangles,
        &mut half_edges,
        &mut faces,
        &mut solids,
    );

    let mut tri_mesh = TriMesh::new();

    let triangles = solids[s01234567]
        .boundary
        .into_iter()
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
