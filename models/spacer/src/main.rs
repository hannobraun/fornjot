use fj::handle_model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = spacer::model(1., 0.5, 1., &mut fj.core);
    handle_model(&model, fj.core)?;
    Ok(())
}
