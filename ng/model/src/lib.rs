#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    fj::Shape::Cube { size: 1.0 }
}
