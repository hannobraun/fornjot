use fj::prelude::*;

fn main() -> anyhow::Result<()> {
    let outer = fj::Circle::from_radius(1.0);
    let inner = fj::Circle::from_radius(0.5);

    let sketch = (outer, inner).difference();
    let spacer = sketch.linear_extrude(1.0);

    fj::run(spacer)?;
    Ok(())
}
