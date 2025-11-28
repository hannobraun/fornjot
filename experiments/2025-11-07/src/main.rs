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

fn model(_: &ViewerHandle) -> TriMesh {
    TriMesh {
        triangles: vec![MeshTriangle {
            inner: Triangle::from_points([
                [0., 0., 0.],
                [1., 0., 0.],
                [0., 1., 0.],
            ]),
            is_internal: false,
            color: Color::default(),
        }],
    }
}
