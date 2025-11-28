use fj::debug::DEBUG_WINDOW;
use fj_interop::TriMesh;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj::viewer::make_viewer_and_spawn_thread(|viewer| {
        DEBUG_WINDOW.initialize(&viewer);
        TriMesh::new()
    })?;

    fj::export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}
