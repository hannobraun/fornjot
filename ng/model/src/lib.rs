#[no_mangle]
pub extern "C" fn model() -> fj::Model {
    fj::Model { cube_size: 1.0 }
}
