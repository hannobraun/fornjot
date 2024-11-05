mod app;
mod export;
mod geometry;
mod model;
mod render;

fn main() -> anyhow::Result<()> {
    let mesh = model::model()?;

    export::export(&mesh)?;
    app::run(mesh)?;

    Ok(())
}
