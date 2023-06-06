use fj::handle_model;

fn main() -> Result<(), fj::Error> {
    let cuboid = cuboid::cuboid(3., 2., 1.);

    // The tolerance makes no difference for this model, as there aren't any
    // curves.
    let tolerance = 1.;
    handle_model(cuboid, tolerance)?;

    Ok(())
}
