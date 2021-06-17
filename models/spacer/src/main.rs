use fj::prelude::*;

fn main() -> anyhow::Result<()> {
    let outer = 50.0;
    let inner = 25.0;
    let height = 25.0;

    let outer = fj::Cylinder::new().with_radius(outer).with_height(height);
    let inner = fj::Cylinder::new().with_radius(inner).with_height(height);

    let spacer = (outer, inner).difference();

    fj::run(spacer)?;

    Ok(())
}
