use std::ops::Deref;

use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let mut services = Services::new();
    let model = star::model(5, 1., 2., 1., &mut services);
    handle_model(model.deref(), services)?;
    Ok(())
}
