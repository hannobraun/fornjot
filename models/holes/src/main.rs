use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = holes::model(0.25, &mut core.services);
    handle_model(&model, core.services)?;
    Ok(())
}
