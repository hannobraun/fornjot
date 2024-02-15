use fj::handle_model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = vertices_indices::model(&mut fj.core);
    handle_model(&model, fj.core)?;
    Ok(())
}
