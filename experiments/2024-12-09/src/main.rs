#![allow(clippy::module_inception)]

mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;
mod storage;
mod topology;
mod view;

fn main() -> anyhow::Result<()> {
    let model = model::model();

    export::export(&model)?;
    app::run(geometry::AnyOp::new(model))?;

    Ok(())
}
