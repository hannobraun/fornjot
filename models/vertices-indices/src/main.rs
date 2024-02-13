use fj::handle_model;

fn main() -> fj::Result {
    let mut core = fj::core::Instance::new();
    let model = vertices_indices::model(&mut core);
    handle_model(&model, core)?;
    Ok(())
}
