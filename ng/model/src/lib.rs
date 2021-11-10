#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    fj::Shape::Cube(fj::Cube { size: 1.0 })
}
