mod mesh;
mod model;
mod export;

fn main() -> anyhow::Result<()> {
    let mesh = model::model()?;
    export::export(mesh.vertices, mesh.triangles)?;
    Ok(())
}
