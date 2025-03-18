#![allow(clippy::module_inception)]

use object::Object;

mod app;
mod export;
mod extra;
mod geometry;
mod math;
mod model;
mod object;
mod operations;
mod render;
mod topology;

fn main() -> anyhow::Result<()> {
    let model = model::model();
    let tri_mesh = model.tri_mesh();

    export::export(&tri_mesh)?;
    app::run(tri_mesh)?;

    Ok(())
}
