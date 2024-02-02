use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = spacer::model(1., 0.5, 1., &mut core.services);
    handle_model(&model, core.services)?;
    Ok(())
}
