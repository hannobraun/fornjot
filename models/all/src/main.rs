use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let model = all::model(&mut Services::new());
    handle_model(model)?;
    Ok(())
}
