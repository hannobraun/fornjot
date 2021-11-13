#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let circle = fj::Circle { radius: 1.0 };

    // TASK: Make hole in circle.
    let footprint = circle;

    let spacer = fj::Sweep {
        shape: footprint.into(),
        length: 1.0,
    };

    spacer.into()
}
