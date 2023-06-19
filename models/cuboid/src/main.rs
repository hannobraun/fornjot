use fj::{core::services::Services, handle_model};

fn main() -> fj::Result {
    let model = cuboid::model(3., 2., 1., &mut Services::new());
    handle_model(model)?;
    Ok(())
}
