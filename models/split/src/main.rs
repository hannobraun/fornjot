fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = split::model(1.0, 0.2, &mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
