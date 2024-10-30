mod model;

fn main() -> anyhow::Result<()> {
    model::model()?;
    Ok(())
}
