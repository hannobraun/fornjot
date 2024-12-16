#![allow(clippy::module_inception)]

mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;
mod storage;
mod view;

fn main() -> anyhow::Result<()> {
    let mut shape = geometry::Shape::default();
    model::model(&mut shape);

    export::export(&shape)?;
    app::run(shape)?;

    Ok(())
}
