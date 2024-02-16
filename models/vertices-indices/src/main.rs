fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = vertices_indices::model(&mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
