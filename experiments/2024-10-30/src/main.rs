mod model;
mod export;

fn main() -> anyhow::Result<()> {
    model::model()?;
    Ok(())
}
