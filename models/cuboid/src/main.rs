use fj::handle_model;

fn main() -> fj::Result {
    let cuboid = cuboid::cuboid(3., 2., 1.);

    handle_model(cuboid)?;

    Ok(())
}
