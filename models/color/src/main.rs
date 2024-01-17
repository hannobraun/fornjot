use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let mut services = Services::new();
    let model = color::model(&mut services);
    handle_model(model, services)?;
    Ok(())
}
