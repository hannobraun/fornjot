use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = cuboid::model([3., 2., 1.], &mut core);
    handle_model(&model, core)?;
    Ok(())
}
