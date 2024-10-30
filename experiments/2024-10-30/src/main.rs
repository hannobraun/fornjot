mod export;
mod mesh;
mod model;

fn main() -> anyhow::Result<()> {
    let mesh = model::model()?;
    export::export(mesh.vertices, mesh.triangles)?;
    Ok(())
}
