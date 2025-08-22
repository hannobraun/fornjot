#![allow(clippy::module_inception)]

mod debug;
mod extra;
mod geometry;
mod handle;
mod model;
mod operations;
mod topology;
#[allow(unused)]
mod viewer;

use debug::DEBUG_WINDOW;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        DEBUG_WINDOW.initialize(&viewer);
        model::model(&viewer)
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}
