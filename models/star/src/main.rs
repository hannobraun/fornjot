fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = star::model(5, 1., 2., 1., &mut fj.core);
    fj.process_model(&model)?;
    Ok(())
}
