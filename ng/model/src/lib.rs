#[no_mangle]
pub extern "C" fn model() -> fj::Shape3d {
    let cube = fj::Cube { size: 1.0 };
    cube.into()
}
