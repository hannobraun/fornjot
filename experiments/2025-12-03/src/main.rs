use fj_interop::{Color, MeshTriangle, TriMesh};

use crate::{
    objects::{
        geometry::{Triangle, Triangles},
        topology::Faces,
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
    let mut solids = Store::default();

    // Push initial vertex.
    let bottom_front_left_vertex = vertices.push([0., 0., 0.]);

    // Sweep initial vertex into lower-left edge.
    let bottom_left_edge = sweep::vertex_to_half_edge(
        bottom_front_left_vertex,
        [0., 1., 0.],
        &mut vertices,
        &mut half_edges,
    );

    // Sweep lower-left edge into bottom face.
    let bottom_face = sweep::half_edge_to_face(
        bottom_left_edge,
        [1., 0., 0.],
        &mut vertices,
        &mut triangles,
        &mut half_edges,
        &mut faces,
    );

    // Sweep bottom face into cube.
    let s01234567 = sweep::face_to_solid(
        bottom_face,
        [0., 0., 1.],
        &mut vertices,
        &mut triangles,
        &mut half_edges,
        &mut faces,
        &mut solids,
    );

    let mut tri_mesh = TriMesh::new();

    let triangles = solids[s01234567]
        .boundary
        .iter()
        .copied()
        .flat_map(|f0123| faces[f0123].triangles.iter().copied())
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
