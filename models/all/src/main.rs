use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let mut services = Services::new();
    let model = all::model(&mut services);
    handle_model(&model, services)?;
    Ok(())
}
