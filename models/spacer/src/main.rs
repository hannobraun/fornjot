use fj::handle_model;

fn main() -> fj::Result {
    let model = spacer::model(1., 0.5, 1.);
    handle_model(model)?;
    Ok(())
}
