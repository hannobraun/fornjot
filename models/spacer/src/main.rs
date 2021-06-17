use fj::prelude::*;

fn main() -> anyhow::Result<()> {
    let height = 25.0;

    let outer = fj::Cylinder::new().with_radius(50.0).with_height(height);
    let inner = fj::Cylinder::new().with_radius(25.0).with_height(height);

    let spacer = (outer, inner).difference();

    fj::run(spacer)?;

    Ok(())
}
