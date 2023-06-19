use fj::handle_model;

fn main() -> fj::Result {
    let model = star::model(5, 1., 2., 1.);
    handle_model(model)?;
    Ok(())
}
