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
    let shape = model::model();

    export::export(&shape)?;
    app::run(geometry::AnyOp::new(shape))?;

    Ok(())
}
