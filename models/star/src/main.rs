use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = star::model(5, 1., 2., 1., &mut core);
    handle_model(&model, core)?;
    Ok(())
}
