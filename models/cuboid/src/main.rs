use fj::handle_model;

fn main() -> fj::Result {
    let cuboid = cuboid::cuboid(3., 2., 1.);

    let tolerance: Option<f64> = None;
    handle_model(cuboid, tolerance)?;

    Ok(())
}
