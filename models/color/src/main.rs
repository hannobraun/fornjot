use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = color::model(&mut core);
    handle_model(&model, core.services)?;
    Ok(())
}
