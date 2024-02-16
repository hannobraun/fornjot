use fj::handle_model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = cuboid::model([3., 2., 1.], &mut fj.core);
    handle_model(&model, &mut fj.core)?;
    Ok(())
}
