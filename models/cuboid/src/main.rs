use fj::{core::algorithms::approx::Tolerance, handle_model};

fn main() -> anyhow::Result<()> {
    let cuboid = cuboid::cuboid(3., 2., 1.);

    // The tolerance makes no difference for this model, as there aren't any
    // curves.
    let tolerance = Tolerance::from_scalar(1.)?;
    handle_model(cuboid, tolerance)?;

    Ok(())
}
