#![allow(clippy::module_inception)]

mod extra;
mod geometry;
mod handle;
mod model;
mod operations;
mod topology;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|viewer| {
        model::model(&viewer)
    })?;

    fj_export::export(tri_mesh.triangles.iter(), "output.3mf")?;

    Ok(())
}
