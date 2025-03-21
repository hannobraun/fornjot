#![allow(clippy::module_inception)]

mod app;
mod export;
mod extra;
mod geometry;
mod handle;
mod math;
mod model;
mod operations;
mod render;
mod topology;

fn main() -> anyhow::Result<()> {
    let tri_mesh = model::model();

    export::export(&tri_mesh, "output.3mf")?;
    app::run(tri_mesh)?;

    Ok(())
}
