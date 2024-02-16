fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = holes::model(0.25, &mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
