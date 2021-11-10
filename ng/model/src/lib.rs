#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    fj::Shape { cube_size: 1.0 }
}
