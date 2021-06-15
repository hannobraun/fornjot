use fj::prelude::*;

fn main() -> anyhow::Result<()> {
    let height = 0.5;

    let outer = fj::Cylinder::new().with_radius(1.0).with_height(height);
    let inner = fj::Cylinder::new().with_radius(0.5).with_height(height);

    let spacer = (outer, inner).difference();

    fj::run(spacer)?;

    Ok(())
}
