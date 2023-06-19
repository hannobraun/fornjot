use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let model = spacer::model(1., 0.5, 1., &mut Services::new());
    handle_model(model)?;
    Ok(())
}
