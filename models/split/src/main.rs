use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = split::model(1.0, 0.2, &mut core);
    handle_model(&model, core.services)?;
    Ok(())
}
