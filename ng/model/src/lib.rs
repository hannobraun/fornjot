#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let cube = fj::Cube { size: 1.0 };
    cube.into()
}
