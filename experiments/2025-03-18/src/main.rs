#![allow(clippy::module_inception)]

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

    export::export(&model)?;
    app::run(model)?;

    Ok(())
}
