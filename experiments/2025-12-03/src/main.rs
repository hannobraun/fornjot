use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::Triangle;

fn main() -> anyhow::Result<()> {
    let vertices = [
        [0., 0., 0.], // v0
        [0., 0., 1.], // v1
        [0., 1., 0.], // v2
        [0., 1., 1.], // v3
        [1., 0., 0.], // v4
        [1., 0., 1.], // v5
        [1., 1., 0.], // v6
        [1., 1., 1.], // v7
    ];

    let triangles = [
        // front
        [0, 4, 5], // t0
        [0, 5, 1], // t1
        // right
        [4, 6, 7], // t2
        [4, 7, 5], // t3
        // back
        [6, 2, 3], // t4
        [6, 3, 7], // t5
        // left
        [2, 0, 1], // t6
        [2, 1, 3], // t7
        // bottom
        [2, 6, 4], // t8
        [2, 4, 0], // t9
        // top
        [1, 5, 7], // t10
        [1, 7, 3], // t11
    ];

    let mut tri_mesh = TriMesh::new();

    for [a, b, c] in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: Triangle::from_points([
                vertices[a],
                vertices[b],
                vertices[c],
            ]),
            is_internal: false,
            color: Color::default(),
        });
    }

    fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        viewer.open_window().display_mesh(tri_mesh);
    })?;

    Ok(())
}
