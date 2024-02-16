fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = color::model(&mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
