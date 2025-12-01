use fj::{debug::DEBUG_WINDOW, viewer::ViewerHandle};
use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::Triangle;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj::viewer::make_viewer_and_spawn_thread(|viewer| {
        DEBUG_WINDOW.initialize(&viewer);
        model(&viewer)
    })?;

    fj::export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model(viewer: &ViewerHandle) -> TriMesh {
    let tri_mesh = TriMesh {
        triangles: vec![
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., -1.],
                    [1., -1., -1.],
                    [-1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., -1.],
                    [1., -1., 1.],
                    [-1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., -1.],
                    [1., 1., -1.],
                    [1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [1., 1., 1.],
                    [1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [-1., 1., -1.],
                    [1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [-1., 1., 1.],
                    [1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [-1., -1., -1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., -1.],
                    [-1., -1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [1., 1., -1.],
                    [-1., -1., -1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [1., -1., -1.],
                    [-1., -1., -1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., 1.],
                    [1., -1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., 1.],
                    [1., 1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
        ],
    };

    viewer.open_window().display_mesh(tri_mesh.clone());

    tri_mesh
}
