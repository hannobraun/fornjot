#![allow(clippy::module_inception)]

mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;
mod ui;

fn main() -> anyhow::Result<()> {
    let mut ops = geometry::Shape::default();
    model::model(&mut ops);

    export::export(&ops)?;
    app::run(ops)?;

    Ok(())
}
