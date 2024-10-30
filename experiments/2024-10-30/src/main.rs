mod export;
mod mesh;
mod model;
mod render;

fn main() -> anyhow::Result<()> {
    let mesh = model::model()?;
    export::export(&mesh)?;
    render::render(&mesh)?;
    Ok(())
}
