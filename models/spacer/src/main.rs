use fj::prelude::*;

fn main() -> anyhow::Result<()> {
    fj::run_model(model)?;

    Ok(())
}

fn model() -> impl Into<fj::Mesh> {
    // TASK: Make it possible to pass those parameters in from the outside, for
    //       example via command-line arguments.
    let outer = 50.0;
    let inner = 25.0;
    let height = 25.0;

    let outer = fj::Cylinder::new().with_radius(outer).with_height(height);
    let inner = fj::Cylinder::new().with_radius(inner).with_height(height);

    let spacer = (outer, inner).difference();

    spacer
}
