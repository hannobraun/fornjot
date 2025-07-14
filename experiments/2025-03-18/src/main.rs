#![allow(clippy::module_inception)]

mod extra;
mod geometry;
mod handle;
mod model;
mod operations;
mod topology;

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj_viewer::make_viewer_and_spawn_thread(|mut viewer| {
        model::model(&mut viewer)
    })?;

    fj_export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}
