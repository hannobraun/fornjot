#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let cube = fj::Sweep {
        shape: fj::Square { size: 1.0 }.into(),
        length: 1.0,
    };
    cube.into()
}
