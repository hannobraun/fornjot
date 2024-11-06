mod app;
mod export;
mod geometry;
mod math;
mod model;
mod render;

fn main() -> anyhow::Result<()> {
    let ops = model::model()?;

    export::export(&ops)?;
    app::run(ops)?;

    Ok(())
}
