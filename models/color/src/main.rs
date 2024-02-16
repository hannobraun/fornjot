use fj::handle_model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = color::model(&mut fj.core);
    handle_model(&model, &mut fj.core)?;
    Ok(())
}
