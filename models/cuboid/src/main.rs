use fj::handle_model;

fn main() -> fj::Result {
    let model = cuboid::cuboid(3., 2., 1.);
    handle_model(model)?;
    Ok(())
}
