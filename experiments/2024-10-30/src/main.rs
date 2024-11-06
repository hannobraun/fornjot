mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;

fn main() -> anyhow::Result<()> {
    let mut ops = geometry::Operations::default();
    model::model(&mut ops)?;

    export::export(&ops)?;
    app::run(ops)?;

    Ok(())
}
