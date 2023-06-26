use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let mut services = Services::new();
    let model = spacer::model(1., 0.5, 1., &mut services);
    handle_model(model, services)?;
    Ok(())
}
