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

    export::export(&model.tri_mesh())?;
    app::run(model.tri_mesh())?;

    Ok(())
}
