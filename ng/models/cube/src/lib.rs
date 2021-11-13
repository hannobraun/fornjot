#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let square = fj::Square { size: 1.0 }.into();

    let cube = fj::Sweep {
        shape: square,
        length: 1.0,
    };

    cube.into()
}
