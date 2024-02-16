fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = all::model(&mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
