#![allow(clippy::module_inception)]

mod extra;
mod geometry;
mod handle;
mod math;
mod model;
mod operations;
mod topology;

fn main() -> anyhow::Result<()> {
    let tri_mesh = model::model();

    fj_export::export(&tri_mesh, "output.3mf")?;
    fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        viewer.display(tri_mesh);
    })?;

    Ok(())
}
