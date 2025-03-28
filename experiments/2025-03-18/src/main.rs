#![allow(clippy::module_inception)]

mod extra;
mod geometry;
mod handle;
mod math;
mod model;
mod operations;
mod topology;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        let tri_mesh = model::model();
        viewer.display(tri_mesh.clone());
        tri_mesh
    })?;

    fj_export::export(&tri_mesh, "output.3mf")?;

    Ok(())
}
