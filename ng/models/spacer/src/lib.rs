#[no_mangle]
pub extern "C" fn model() -> fj::Shape {
    let circle = fj::Circle { radius: 1.0 };

    // TASK: Make hole in circle.
    // TASK: Extrude 2D shape into 3D shape..

    circle.into()
}
